<template>
  <div class="VueToNuxtLogo">
    <el-table :data="tableData" :default-sort="{prop: 'email', order: 'descending'}" style="width: 100%">
      <el-table-column class-name="filter-column" prop="email" label="email" sortable>
      </el-table-column>
      <el-table-column prop="firstname" label="First Name">
      </el-table-column>
      <el-table-column prop="lastname" label="Last Name" >
      </el-table-column>
    </el-table>
  </div>
</template>

<script>
  import Vue from 'vue'
  import { ElTable, ElTableColumn } from 'element-table'
  import 'element-theme-chalk/lib/table.css'

  export default {
    mounted: function () {
      fetch('http://localhost:3100/tabledata', {
        mode: 'cors'
      }).then((ret) => ret.json())
        .then((ret) => {
          Vue.set(this, 'tableData', ret.data)
        })
        .catch((err) => {
          console.error(err)
        })
    },
    data: function () {
      return {
        tableData: []
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
