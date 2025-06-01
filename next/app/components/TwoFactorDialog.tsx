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
} from '@mui/material';
import { useState } from 'react';

interface TwoFactorDialogProps {
  open: boolean;
  onClose: () => void;
  onSubmit: (code: string) => void;
  loading?: boolean;
}

export default function TwoFactorDialog({
  open,
  onClose,
  onSubmit,
  loading = false,
}: TwoFactorDialogProps) {
  const [code, setCode] = useState('');

  const handleSubmit = () => {
    if (code.trim()) {
      onSubmit(code.trim());
    }
  };

  const handleClose = () => {
    setCode('');
    onClose();
  };

  return (
    <Dialog open={open} onClose={handleClose} maxWidth="sm" fullWidth>
      <DialogTitle>二步验证</DialogTitle>
      <DialogContent>
        <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
          请输入您的二步验证码
        </Typography>
        <TextField
          autoFocus
          label="验证码"
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
          取消
        </Button>
        <Button
          onClick={handleSubmit}
          variant="contained"
          disabled={!code.trim() || loading}>
          {loading ? '验证中...' : '验证'}
        </Button>
      </DialogActions>
    </Dialog>
  );
}
