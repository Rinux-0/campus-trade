<script setup lang="ts">
import { ref, onMounted } from 'vue'
import api from '../api'

interface User { user_id: string; user_name: string; phone: string }

const users = ref<User[]>([])
const showForm = ref(false)
const form = ref({ user_name: '', phone: '' })
const editUserId = ref('')
const editName = ref('')
const editPhone = ref('')
const deleteMsg = ref('')

async function load() { const { data } = await api.get('/users'); users.value = data }

async function create() { await api.post('/users', form.value); showForm.value = false; load() }

async function update(userId: string) {
  await api.put(`/users/${userId}`, { user_name: editName.value || undefined, phone: editPhone.value || undefined })
  editUserId.value = ''; load()
}

async function remove(userId: string) {
  deleteMsg.value = ''
  try { await api.delete(`/users/${userId}`); load() }
  catch (e: any) { deleteMsg.value = e.response?.data?.error || e.message }
}

onMounted(load)
</script>

<template>
  <h2>用户管理</h2>

  <button @click="showForm = !showForm" style="margin-bottom:1rem;">{{ showForm ? '取消' : '新增用户' }}</button>

  <form v-if="showForm" @submit.prevent="create" style="margin:1rem 0;padding:1rem;border:1px solid var(--pico-muted-border-color);border-radius:var(--pico-border-radius);">
    <input v-model="form.user_name" placeholder="姓名" required />
    <input v-model="form.phone" placeholder="手机号" required />
    <button type="submit">提交</button>
  </form>

  <table>
    <thead><tr><th>用户ID</th><th>姓名</th><th>手机号</th><th>操作</th></tr></thead>
    <tbody>
      <tr v-for="u in users" :key="u.user_id">
        <td>{{ u.user_id }}</td>
        <td>
          <template v-if="editUserId !== u.user_id">{{ u.user_name }}</template>
          <template v-else><input v-model="editName" /></template>
        </td>
        <td>
          <template v-if="editUserId !== u.user_id">{{ u.phone }}</template>
          <template v-else><input v-model="editPhone" /></template>
        </td>
        <td style="white-space:nowrap;">
          <template v-if="editUserId !== u.user_id">
            <button class="outline" style="margin-right:0.5rem;" @click="editUserId = u.user_id; editName = u.user_name; editPhone = u.phone">编辑</button>
            <button class="secondary" @click="remove(u.user_id)">注销</button>
          </template>
          <template v-else>
            <button style="margin-right:0.5rem;" @click="update(u.user_id)">确认</button>
            <button class="secondary" @click="editUserId = ''">取消</button>
          </template>
        </td>
      </tr>
    </tbody>
  </table>
  <p v-if="deleteMsg"><mark>{{ deleteMsg }}</mark></p>
</template>
