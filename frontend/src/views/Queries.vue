<script setup lang="ts">
import { ref, onMounted, type Ref } from 'vue'
import api from '../api'

interface Item { item_id: string; item_name: string; category: string; price: number; status: number; seller_id: string }

const allFields = ['item_id', 'item_name', 'category', 'price', 'status', 'seller_id',
                   'order_id', 'buyer_id', 'buyer_name', 'order_date'] as const
type Field = typeof allFields[number]

const fieldLabels: Record<Field, string> = {
  item_id: '商品ID', item_name: '商品名', category: '分类', price: '价格', status: '状态',
  seller_id: '卖家ID', order_id: '订单ID', buyer_id: '买家ID', buyer_name: '买家名', order_date: '日期',
}

const presets: Record<string, Field[]> = {
  '商品信息': ['item_id', 'item_name', 'category', 'price', 'status', 'seller_id'],
  '订单摘要': ['item_id', 'item_name', 'buyer_name', 'order_date'],
  '完整信息': [...allFields],
}

const fStatus = ref<number | null>(null)
const fCategory = ref<string[]>([])
const fSeller = ref<string[]>([])
const fBuyer = ref<string[]>([])
const fItemId = ref('')
const fMinPrice = ref<number | null>(null)
const fMaxPrice = ref<number | null>(null)
const fDateFrom = ref('')
const fDateTo = ref('')
const visibleFields = ref<Field[]>([...presets['商品信息']])
const allCategories = ref<string[]>([])
const allSellers = ref<string[]>([])
const allBuyers = ref<string[]>([])
const results = ref<any[]>([])
const resultTitle = ref('')

onMounted(async () => {
  try {
    const { data: items } = await api.get('/items')
    allCategories.value = [...new Set((items as Item[]).map(i => i.category))].sort()
    allSellers.value = [...new Set((items as Item[]).map(i => i.seller_id))].sort()
    const { data: users } = await api.get('/users')
    allBuyers.value = (users as any[]).map(u => u.user_id).sort()
  } catch {}
})

function toggleCat(c: string) { toggle(fCategory, c) }
function toggleSeller(s: string) { toggle(fSeller, s) }
function toggleBuyer(b: string) { toggle(fBuyer, b) }
function toggle(arr: Ref<string[]>, val: string) {
  const i = arr.value.indexOf(val)
  if (i >= 0) arr.value.splice(i, 1); else arr.value.push(val)
}
function toggleField(f: Field) {
  const i = visibleFields.value.indexOf(f)
  if (i >= 0) visibleFields.value.splice(i, 1); else visibleFields.value.push(f)
}
function applyPreset(name: string) { visibleFields.value = [...presets[name]] }

async function doSearch() {
  const p: Record<string, any> = {}
  if (fStatus.value !== null) p.status = fStatus.value
  if (fCategory.value.length) p.category = fCategory.value.join(',')
  if (fSeller.value.length) p.seller_id = fSeller.value.join(',')
  if (fBuyer.value.length) p.buyer_id = fBuyer.value.join(',')
  if (fItemId.value.trim()) p.item_id = fItemId.value.trim()
  if (fMinPrice.value !== null) p.min_price = fMinPrice.value
  if (fMaxPrice.value !== null) p.max_price = fMaxPrice.value
  if (fDateFrom.value.trim()) p.date_from = fDateFrom.value.trim()
  if (fDateTo.value.trim()) p.date_to = fDateTo.value.trim()
  try {
    const { data } = await api.get('/queries/search', { params: p })
    results.value = data; resultTitle.value = `查询结果 (${data.length} 条)`
  } catch (e: any) { resultTitle.value = '查询失败: ' + (e?.message || '未知错误'); results.value = [] }
}

function showCol(k: string) { return fieldLabels[k as Field] || k }
function showVal(item: any, k: string) {
  if (k === 'status') return item[k] === 1 ? '已售' : '未售'
  const v = item[k]
  return v === null ? '-' : v
}
</script>

<template>
  <h2>自由查询</h2>

  <div class="grid">
    <select v-model="fStatus" style="width:auto;">
      <option :value="null">全部</option>
      <option :value="0">未售</option>
      <option :value="1">已售</option>
    </select>
    <input v-model="fItemId" placeholder="商品ID (逗号分隔)" />
    <input v-model.number="fMinPrice" type="number" placeholder="最低价" />
    <input v-model.number="fMaxPrice" type="number" placeholder="最高价" />
    <input v-model="fDateFrom" placeholder="日期起" />
    <input v-model="fDateTo" placeholder="日期止" />
  </div>

  <fieldset style="margin-top:0.5rem;">
    <legend>分类</legend>
    <label v-for="c in allCategories" :key="c" style="display:inline-block;margin-right:0.5rem;">
      <input type="checkbox" @change="toggleCat(c)" :checked="fCategory.includes(c)" /> {{ c }}
    </label>
  </fieldset>
  <fieldset>
    <legend>卖家</legend>
    <label v-for="s in allSellers" :key="s" style="display:inline-block;margin-right:0.5rem;">
      <input type="checkbox" @change="toggleSeller(s)" :checked="fSeller.includes(s)" /> {{ s }}
    </label>
  </fieldset>
  <fieldset>
    <legend>买家</legend>
    <label v-for="b in allBuyers" :key="b" style="display:inline-block;margin-right:0.5rem;">
      <input type="checkbox" @change="toggleBuyer(b)" :checked="fBuyer.includes(b)" /> {{ b }}
    </label>
  </fieldset>

  <button @click="doSearch">查询</button>

  <details open style="margin:0.5rem 0;">
    <summary>显示列</summary>
    <div class="grid" style="margin-top:0.5rem;">
      <button v-for="(_, name) in presets" :key="name" @click="applyPreset(name)" class="outline" style="font-size:0.85em;">{{ name }}</button>
    </div>
    <div style="margin-top:0.5rem;">
      <label v-for="f in allFields" :key="f" style="display:inline-block;margin-right:0.5rem;">
        <input type="checkbox" @change="toggleField(f)" :checked="visibleFields.includes(f)" /> {{ fieldLabels[f] }}
      </label>
    </div>
  </details>

  <table v-if="results.length" style="margin-top:1rem;">
    <caption>{{ resultTitle }}</caption>
    <thead><tr><th v-for="k in visibleFields" :key="k">{{ showCol(k) }}</th></tr></thead>
    <tbody>
      <tr v-for="(item, i) in results" :key="i">
        <td v-for="k in visibleFields" :key="k">{{ showVal(item, k) }}</td>
      </tr>
    </tbody>
  </table>
  <p v-else-if="resultTitle"><small>{{ resultTitle }}</small></p>
</template>
