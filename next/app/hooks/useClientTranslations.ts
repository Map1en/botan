'use client';

import { useState, useEffect } from 'react';

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
  }, [locale]);

  const t = (key: string): string => {
    const keys = key.split('.');
    let value = messages;

    for (const k of keys) {
      value = value?.[k];
      if (value === undefined) break;
    }

    return typeof value === 'string' ? value : key;
  };

  return { t, locale, setLocale };
}
