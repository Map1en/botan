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

interface EmailOtpDialogProps {
  open: boolean;
  onClose: () => void;
  onSubmit: (otp: string) => void;
  loading?: boolean;
}

export default function EmailOtpDialog({
  open,
  onClose,
  onSubmit,
  loading = false,
}: EmailOtpDialogProps) {
  const [otp, setOtp] = useState('');

  const handleSubmit = () => {
    if (otp.trim()) {
      onSubmit(otp.trim());
    }
  };

  const handleClose = () => {
    setOtp('');
    onClose();
  };

  return (
    <Dialog open={open} onClose={handleClose} maxWidth="sm" fullWidth>
      <DialogTitle>邮箱验证</DialogTitle>
      <DialogContent>
        <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
          我们已向您的邮箱发送了验证码，请输入收到的验证码
        </Typography>
        <TextField
          autoFocus
          label="验证码"
          type="text"
          fullWidth
          variant="outlined"
          value={otp}
          onChange={(e) => setOtp(e.target.value)}
          onKeyPress={(e) => {
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
          disabled={!otp.trim() || loading}>
          {loading ? '验证中...' : '验证'}
        </Button>
      </DialogActions>
    </Dialog>
  );
}
