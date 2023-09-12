<script setup>
//  $toast.show('Logging in...')
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';
let permissionGranted = await isPermissionGranted();
if (!permissionGranted) {
  const permission = await requestPermission();
  permissionGranted = permission === 'granted';
}
if (permissionGranted) {
  sendNotification('Tauri is awesome!');
  sendNotification({ title: 'TAURI', body: 'Tauri is awesome!' });
}
</script>
<template>
    <div>
      <div>Nuxt Vite PWA</div>
      <NuxtLink to="/">
        About
      </NuxtLink>
    </div>
  </template>