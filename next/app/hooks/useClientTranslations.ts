'use client';

import { useState, useEffect, useCallback } from 'react';

interface Messages {
  [key: string]: any;
}

const translations: { [locale: string]: Messages } = {
  'zh-CN': require('../../messages/zh-CN.json'),
  'en-US': require('../../messages/en-US.json'),
};

export function useClientTranslations(namespace?: string) {
  const [locale, setLocale] = useState('zh-CN');
  const [messages, setMessages] = useState<Messages>(translations['zh-CN']);

  useEffect(() => {
    const savedLocale = localStorage.getItem('locale') || 'zh-CN';
    if (savedLocale !== locale) {
      setLocale(savedLocale);
      setMessages(translations[savedLocale] || translations['zh-CN']);
    }

    // 监听语言切换事件
    const handleLanguageChange = (event: CustomEvent) => {
      const newLocale = event.detail.locale;
      setLocale(newLocale);
      setMessages(translations[newLocale] || translations['zh-CN']);
    };

    window.addEventListener(
      'languageChanged',
      handleLanguageChange as EventListener,
    );

    return () => {
      window.removeEventListener(
        'languageChanged',
        handleLanguageChange as EventListener,
      );
    };
  }, [locale]);

  const t = useCallback(
    (key: string): string => {
      const keys = key.split('.');
      let value = messages;

      for (const k of keys) {
        value = value?.[k];
        if (value === undefined) break;
      }

      return typeof value === 'string' ? value : key;
    },
    [messages],
  );

  return { t, locale, setLocale };
}
