// Copyright 2025 Genouka. All rights reserved.
(function () {
  if (!window.__TAURI_INTERNALS__) return;   // 仅 Tauri 环境生效

  window.Android = {
    readFile11: () => window.__TAURI__.core.invoke('read_file_11', { href: location.href }),
    writeSyncT: (text) => window.__TAURI__.core.invoke('write_sync_t', { href: location.href, text }),
    closeDialogT: () => {
      const base = 'https://dreditorcn.genouka.top';
      const platform = window.Android.getPlatform();
      const ok = platform === `${base}/win/success.html?reason=closeDialogT`;
      location.href = ok;
    },
    getPlatform: () => window.__TAURI__.core.invoke('get_platform'),
    getSaveDirectory: () => window.__TAURI__.core.invoke('get_save_directory')
  };
})();