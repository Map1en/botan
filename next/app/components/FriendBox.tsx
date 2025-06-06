'use client';

import React, { useState, useRef, useEffect } from 'react';
import { Paper, IconButton, Typography, Box, Fab } from '@mui/material';
import {
  Close as CloseIcon,
  DragIndicator as DragIcon,
  People as PeopleIcon,
} from '@mui/icons-material';
import { useTheme } from '../../theme/ThemeContext';

interface DraggableFloatingBoxProps {
  children?: React.ReactNode;
  title?: string;
  initialPosition?: { x: number; y: number };
  onClose?: () => void;
}

export default function DraggableFloatingBox({
  children,
  title,
  initialPosition = { x: 20, y: 100 },
  onClose,
}: DraggableFloatingBoxProps) {
  const { mode } = useTheme();
  const [position, setPosition] = useState(initialPosition);
  const [isDragging, setIsDragging] = useState(false);
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 });
  const [isMinimized, setIsMinimized] = useState(false);
  const boxRef = useRef<HTMLDivElement>(null);

  const handleMouseDown = (e: React.MouseEvent) => {
    if (!boxRef.current) return;

    const rect = boxRef.current.getBoundingClientRect();
    setDragOffset({
      x: e.clientX - rect.left,
      y: e.clientY - rect.top,
    });
    setIsDragging(true);
  };

  const handleMouseMove = (e: MouseEvent) => {
    if (!isDragging) return;

    const newX = e.clientX - dragOffset.x;
    const newY = e.clientY - dragOffset.y;

    const maxX = window.innerWidth - (boxRef.current?.offsetWidth || 300);
    const maxY = window.innerHeight - (boxRef.current?.offsetHeight || 200);

    setPosition({
      x: Math.max(0, Math.min(newX, maxX)),
      y: Math.max(0, Math.min(newY, maxY)),
    });
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  useEffect(() => {
    if (isDragging) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
    }

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging, dragOffset]);

  const toggleMinimize = () => {
    setIsMinimized(!isMinimized);
  };

  return (
    <Box
      ref={boxRef}
      sx={{
        position: 'fixed',
        left: position.x,
        top: position.y,
        zIndex: 1300,
        cursor: isDragging ? 'grabbing' : 'default',
        transition: isDragging ? 'none' : 'all 0.2s ease-in-out',
      }}>
      <Paper
        elevation={8}
        sx={{
          width: isMinimized ? 'auto' : 320,
          bgcolor: mode === 'dark' ? 'grey.800' : 'background.paper',
          border:
            mode === 'dark'
              ? '1px solid rgba(255,255,255,0.1)'
              : '1px solid rgba(0,0,0,0.1)',
          borderRadius: 2,
          overflow: 'hidden',
          transform: isMinimized ? 'scale(0.9)' : 'scale(1)',
        }}>
        <Box
          sx={{
            display: 'flex',
            alignItems: 'center',
            padding: '8px 12px',
            bgcolor: mode === 'dark' ? 'grey.700' : 'grey.100',
            cursor: 'grab',
            userSelect: 'none',
            '&:active': {
              cursor: 'grabbing',
            },
          }}
          onMouseDown={handleMouseDown}>
          <DragIcon
            sx={{
              mr: 1,
              fontSize: 16,
              color: mode === 'dark' ? 'grey.400' : 'grey.600',
            }}
          />
          <Typography variant="subtitle2" sx={{ flex: 1, fontSize: 14 }}>
            {title}
          </Typography>

          <IconButton
            size="small"
            onClick={toggleMinimize}
            sx={{ p: 0.5, mr: 0.5 }}>
            <Typography sx={{ fontSize: 12, fontWeight: 'bold' }}>
              {isMinimized ? '□' : '—'}
            </Typography>
          </IconButton>

          {onClose && (
            <IconButton size="small" onClick={onClose} sx={{ p: 0.5 }}>
              <CloseIcon sx={{ fontSize: 16 }} />
            </IconButton>
          )}
        </Box>

        {!isMinimized && <Box sx={{ p: 2 }}>{children}</Box>}
      </Paper>
    </Box>
  );
}

export function FloatingActionButton({ onClick }: { onClick: () => void }) {
  const { mode } = useTheme();

  return (
    <Fab
      color="primary"
      onClick={onClick}
      sx={{
        position: 'fixed',
        bottom: 24,
        right: 24,
        zIndex: 1200,
        bgcolor: mode === 'dark' ? 'primary.dark' : 'primary.main',
        '&:hover': {
          bgcolor: mode === 'dark' ? 'primary.main' : 'primary.dark',
          transform: 'scale(1.1)',
        },
        transition: 'all 0.2s ease-in-out',
      }}>
      <PeopleIcon />
    </Fab>
  );
}
