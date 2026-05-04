<script setup lang="ts">
import { ref } from 'vue'
import api from '../api'

const msg = ref('')

async function reset() {
  try { await api.post('/reset'); msg.value = '数据已重置' }
  catch (e: any) { msg.value = '失败: ' + (e?.message || e.message) }
}
</script>

<template>
  <section style="text-align:center;padding:3rem 0 2rem;">
    <h1>校园二手交易平台</h1>
    <p style="max-width:32rem;margin:0 auto;color:var(--pico-muted-color);">Rust Axum + Vue 3 + PostgreSQL — 数据库大作业，CRUD、事务、自由查询、视图、聚合统计全覆盖。</p>
    <div style="margin-top:1.5rem;display:flex;gap:0.75rem;justify-content:center;">
      <a href="/api" target="_blank" role="button">API 文档</a>
      <a href="https://github.com" target="_blank" role="button" class="outline">查看源码</a>
    </div>
  </section>

  <section style="padding:2rem 0;">
    <h3 style="text-align:center;">功能亮点</h3>
    <div class="grid" style="margin-top:1rem;">
      <article>
        <h5>🛒 商品交易</h5>
        <p>CRUD + 状态筛选 + 事务防御性购买</p>
      </article>
      <article>
        <h5>🔍 自由查询</h5>
        <p>多维度任意组合 + 列可见性可控</p>
      </article>
      <article>
        <h5>📊 聚合统计</h5>
        <p>总数/分类/均价/TOP，一键刷新</p>
      </article>
      <article>
        <h5>👤 用户管理</h5>
        <p>ID 空缺自动补位 + 级联注销</p>
      </article>
    </div>
  </section>

  <section style="padding:2rem 0;background:var(--pico-card-background-color);border-radius:var(--pico-border-radius);">
    <div class="grid">
      <div style="text-align:center;">
        <h3>技术架构</h3>
        <table style="margin:0.5rem auto 0;width:auto;display:table;">
          <thead><tr><th>层</th><th>技术栈</th></tr></thead>
          <tbody>
            <tr><td>前端</td><td>Vue 3 + TS + Vite + Pico.css</td></tr>
            <tr><td>后端</td><td>Rust Axum + sqlx + utoipa</td></tr>
            <tr><td>数据库</td><td>PostgreSQL (3表+2视图)</td></tr>
            <tr><td>版本控制</td><td>Jujutsu (jj) + GitHub</td></tr>
            <tr><td>部署运维</td><td>Docker + Sealos 云平台</td></tr>
          </tbody>
        </table>
      </div>
      <div style="text-align:center;">
        <h3>设计原则</h3>
        <ul style="margin:0.5rem auto 0;display:inline-block;text-align:left;">
          <li>自顶向下设计，自底向上实现</li>
          <li>模块化解耦，骨架最小集优先</li>
          <li>前后端分离，热重载调试</li>
          <li>单一二进制交付，精简运行时</li>
        </ul>
      </div>
    </div>
  </section>

  <section style="padding:1rem 0 3rem;text-align:center;">
    <hr />
    <button @click="reset" class="secondary">重置为原始数据</button>
    <p v-if="msg" style="margin-top:0.5rem;"><small>{{ msg }}</small></p>
  </section>
</template>
