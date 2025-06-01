'use client';

import React from 'react';
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Button,
  Typography,
  Alert,
} from '@mui/material';
import { useState } from 'react';
import { useClientTranslations } from '../hooks/useClientTranslations';

interface TwoFactorDialogProps {
  open: boolean;
  onClose: () => void;
  onSubmit: (code: string) => void;
  loading?: boolean;
  type: 'email' | '2fa';
  error?: string;
  onClearError?: () => void;
}

export default function TwoFactorDialog({
  open,
  onClose,
  onSubmit,
  loading = false,
  type,
  error,
  onClearError,
}: TwoFactorDialogProps) {
  const { t } = useClientTranslations();

  const [code, setCode] = useState('');

  const handleSubmit = () => {
    if (code.trim()) {
      onSubmit(code.trim());
    }
  };

  const handleClose = () => {
    setCode('');
    if (onClearError) {
      onClearError();
    }
    onClose();
  };

  const handleClearError = () => {
    if (onClearError) {
      onClearError();
    }
  };

  const getTitle = () => {
    return type === 'email' ? t('twoFactor.emailTitle') : t('twoFactor.title');
  };

  const getDescription = () => {
    return type === 'email'
      ? t('twoFactor.descriptions.email')
      : t('twoFactor.descriptions.totp');
  };

  return (
    <Dialog open={open} onClose={handleClose} maxWidth="sm" fullWidth>
      <DialogTitle>{getTitle()}</DialogTitle>
      <DialogContent>
        <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
          {getDescription()}
        </Typography>

        {error && (
          <Alert severity="error" sx={{ mb: 2 }} onClose={handleClearError}>
            {error}
          </Alert>
        )}

        <TextField
          autoFocus
          label={t('twoFactor.code')}
          type="text"
          fullWidth
          variant="outlined"
          value={code}
          onChange={(e) => setCode(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') {
              handleSubmit();
            }
          }}
          disabled={loading}
        />
      </DialogContent>
      <DialogActions>
        <Button onClick={handleClose} disabled={loading}>
          {t('common.cancel')}
        </Button>
        <Button
          onClick={handleSubmit}
          variant="contained"
          disabled={!code.trim() || loading}>
          {loading ? t('twoFactor.verifying') : t('twoFactor.verify')}
        </Button>
      </DialogActions>
    </Dialog>
  );
}
