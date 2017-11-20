<template>
  <div>
    <el-pagination layout="prev, pager, next" @current-change="changePage" :page-size="pageSize" :total="dataLength" />
    <el-table :data="filteredData" :default-sort="{prop: 'email', order: 'descending'}" style="width: 100%">
      <el-table-column type="index" sortable />
      <el-table-column class-name="filter-column" prop="email" label="email" sortable />
      <el-table-column prop="firstname" label="First Name" sortable />
      <el-table-column prop="lastname" label="Last Name" sortable />
    </el-table>
  </div>
</template>

<script>
  import Vue from 'vue'
  import {
    Table as ElTable,
    TableColumn as ElTableColumn,
    Pagination as ElPagination
  } from 'element-ui'
  import 'element-theme-chalk/lib/table.css'
  import 'element-theme-chalk/lib/pagination.css'
  import 'element-theme-chalk/lib/icon.css'

  export default {
    mounted () {
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
    data () {
      return {
        tableData: [],
        currentPage: 1
      }
    },
    components: {
      'el-pagination': ElPagination,
      'el-table': ElTable,
      'el-table-column': ElTableColumn
    },
    methods: {
      changePage (value) {
        Vue.set(this, 'currentPage', value)
      }
    },
    computed: {
      dataLength () {
        return this.tableData.length
      },
      filteredData () {
        return this.tableData.slice((this.currentPage - 1) * this.pageSize, this.currentPage * this.pageSize)
      },
      pageSize () {
        return 20
      }
    }
  }
</script>

<style>
</style>
