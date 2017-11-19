<template>
  <div class="VueToNuxtLogo">
    <el-table :data="tableData" :default-sort="{prop: 'date', order: 'descending'}" style="width: 100%">
      <el-table-column class-name="filter-column" prop="date" label="Date" sortable>
      </el-table-column>
      <el-table-column prop="name" label="Name" :filters="[{ text: 'Home', value: 'hoge2' }, { text: 'Office', value: 'Office' }]"
        :filter-method="filterTag" filter-placement="bottom-end">
      </el-table-column>
      <el-table-column prop="address" label="Address" >
      </el-table-column>
    </el-table>
  </div>
</template>

<script>
  import { ElTable, ElTableColumn } from 'element-table'
  import 'element-theme-chalk/lib/table.css'

  export default {
    data: function () {
      return {
        tableData: [
          {
            date: '2017/4/2',
            name: 'hoge2',
            address: 'mi2yajo'
          },
          {
            date: '2017/5/2',
            name: 'hoge',
            address: 'mi3yajo'
          },
          {
            date: '2017/3/2',
            name: 'hoge',
            address: 'mi1yajo'
          }
        ]
      }
    },
    components: {
      'el-table': ElTable,
      'el-table-column': ElTableColumn
    }
  }
</script>

<style>
</style>
