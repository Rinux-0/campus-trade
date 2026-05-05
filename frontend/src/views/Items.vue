<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import api from '../api'

interface Item { item_id: string; item_name: string; category: string; price: number; status: number; seller_id: string }

const items = ref<Item[]>([])
const statusFilter = ref<number | null>(null)
const showForm = ref(false)
const form = ref({ item_id: '', item_name: '', category: '', price: 0, seller_id: '' })
const editItemId = ref('')
const editPrice = ref(0)
const buyItemId = ref('')
const buyBuyerId = ref('')
const buyMsg = ref('')
const deleteMsg = ref('')

const filtered = computed(() => statusFilter.value === null ? items.value : items.value.filter(i => i.status === statusFilter.value))

async function load() { const { data } = await api.get('/items'); items.value = data }
async function create() { await api.post('/items', form.value); showForm.value = false; load() }
async function updatePrice() { await api.put(`/items/${editItemId.value}/price`, { price: editPrice.value }); editItemId.value = ''; load() }

async function remove(itemId: string) {
  deleteMsg.value = ''
  try { await api.delete(`/items/${itemId}`); load() }
  catch (e: any) { deleteMsg.value = e.response?.data?.error || e.message }
}

function buy(itemId: string) { buyMsg.value = ''; buyItemId.value = itemId }

async function confirmBuy() {
  try {
    await api.post('/orders/buy', { item_id: buyItemId.value, buyer_id: buyBuyerId.value, order_date: new Date().toISOString().slice(0, 10) })
    buyItemId.value = ''; buyBuyerId.value = ''; load()
  } catch (e: any) { buyMsg.value = e.response?.data?.error || e.message }
}

// Aggregation
const aggLines = ref<string[]>([])
const aggTitle = ref('')
async function aggAll() {
  try {
    const [c, cat, avg, top] = await Promise.all([api.get('/queries/count'), api.get('/queries/category-count'), api.get('/queries/avg-price'), api.get('/queries/top-seller')])
    const cats = cat.data.map((x: any) => `${x.category}(${x.count})`).join(', ')
    aggLines.value = [`商品总数: ${c.data.total}`, `分类统计: ${cats}`, `平均价格: ${avg.data.avg_price}`, `发布最多用户: ${top.data.seller_id} (${top.data.count}件)`]
    aggTitle.value = '聚合统计'
  } catch (e: any) { aggTitle.value = '统计失败'; aggLines.value = [] }
}

onMounted(() => { load(); aggAll() })
</script>

<template>
  <h2>聚合统计</h2>
  <button class="outline" @click="aggAll">刷新</button>
  <ul v-if="aggLines.length" style="margin-top:0.5rem;">
    <li v-for="line in aggLines" :key="line">{{ line }}</li>
  </ul>

  <h2>商品管理</h2>

  <div class="grid" style="margin-bottom:1rem;">
    <button @click="showForm = !showForm">{{ showForm ? '取消' : '新增商品' }}</button>
    <select v-model="statusFilter" style="width:auto;">
      <option :value="null">全部</option>
      <option :value="0">未售</option>
      <option :value="1">已售</option>
    </select>
    <small>共 {{ filtered.length }} 件</small>
  </div>

  <form v-if="showForm" @submit.prevent="create" style="margin:1rem 0;padding:1rem;border:1px solid var(--pico-muted-border-color);border-radius:var(--pico-border-radius);">
    <input v-model="form.item_id" placeholder="商品ID" required />
    <input v-model="form.item_name" placeholder="商品名" required />
    <input v-model="form.category" placeholder="分类" required />
    <input v-model.number="form.price" type="number" placeholder="价格" required />
    <input v-model="form.seller_id" placeholder="卖家ID" required />
    <button type="submit">提交</button>
  </form>

  <div v-if="buyItemId" style="margin:1rem 0;padding:1rem;border:2px solid var(--pico-primary-focus);border-radius:var(--pico-border-radius);">
    <h5>购买商品: {{ buyItemId }}</h5>
    <div class="grid">
      <input v-model="buyBuyerId" placeholder="买家ID" />
      <button @click="confirmBuy">确认购买</button>
      <button class="secondary" @click="buyItemId = ''">取消</button>
    </div>
    <p v-if="buyMsg"><mark>{{ buyMsg }}</mark></p>
  </div>

  <table>
    <thead><tr><th>ID</th><th>名称</th><th>分类</th><th>价格</th><th>状态</th><th>卖家</th><th>操作</th></tr></thead>
    <tbody>
      <tr v-for="item in filtered" :key="item.item_id">
        <td>{{ item.item_id }}</td>
        <td>{{ item.item_name }}</td>
        <td>{{ item.category }}</td>
        <td>
          <template v-if="editItemId !== item.item_id">{{ item.price }}</template>
          <template v-else>
            <input v-model.number="editPrice" type="number" style="width:6rem;" />
            <button @click="updatePrice">√</button>
            <button class="secondary" @click="editItemId = ''">×</button>
          </template>
        </td>
        <td>{{ item.status === 1 ? '已售' : '未售' }}</td>
        <td>{{ item.seller_id }}</td>
        <td style="white-space:nowrap;">
          <button class="outline" v-if="item.status === 0" @click="editItemId = item.item_id; editPrice = item.price">改价</button>
          <button class="outline" v-if="item.status === 0" style="margin-left:0.25rem;" @click="buy(item.item_id)">购买</button>
          <button class="secondary" v-if="item.status === 0" style="margin-left:0.25rem;" @click="remove(item.item_id)">删除</button>
        </td>
      </tr>
    </tbody>
  </table>
  <p v-if="deleteMsg"><mark>{{ deleteMsg }}</mark></p>
</template>
