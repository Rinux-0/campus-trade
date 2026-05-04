<script setup lang="ts">
import { ref, onMounted } from 'vue'
import api from '../api'

interface Order { order_id: string; item_id: string; buyer_id: string; order_date: string }
const orders = ref<Order[]>([])
onMounted(async () => { const { data } = await api.get('/orders'); orders.value = data })
</script>

<template>
  <h2>订单列表</h2>
  <table>
    <thead><tr><th>订单ID</th><th>商品ID</th><th>买家ID</th><th>日期</th></tr></thead>
    <tbody>
      <tr v-for="o in orders" :key="o.order_id">
        <td>{{ o.order_id }}</td><td>{{ o.item_id }}</td><td>{{ o.buyer_id }}</td><td>{{ o.order_date }}</td>
      </tr>
    </tbody>
  </table>
</template>
