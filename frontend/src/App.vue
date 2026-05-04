<script setup lang="ts">
</script>

<template>
  <nav class="container-fluid">
    <ul>
      <li><strong>校园二手交易</strong></li>
    </ul>
    <ul>
      <li><router-link to="/">主页</router-link></li>
      <li><router-link to="/users">用户</router-link></li>
      <li><router-link to="/items">商品</router-link></li>
      <li><router-link to="/orders">订单</router-link></li>
      <li><router-link to="/queries">查询</router-link></li>
    </ul>
  </nav>
  <main class="container" style="padding-top:1.5rem;">
    <router-view />
  </main>
</template>

<style>
html {
  background: linear-gradient(160deg, #060910 0%, #0b101c 25%, #080e18 55%, #0d1322 80%, #070a12 100%);
  background-attachment: fixed;
  min-height: 100vh;
}

body {
  background: transparent;
}

nav {
  background: linear-gradient(180deg, rgba(255,255,255,0.03) 0%, transparent 100%);
  border-bottom: 1px solid rgba(255,255,255,0.06);
  backdrop-filter: blur(12px);
}

article {
  background: linear-gradient(135deg, rgba(255,255,255,0.04) 0%, rgba(255,255,255,0.02) 100%);
  box-shadow: 0 1px 3px rgba(0,0,0,0.2);
}

h1, h2, h3 {
  letter-spacing: -0.02em;
}

[role="button"] {
  transition: all 0.15s ease;
}
</style>
