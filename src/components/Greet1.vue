<template>
  <div>
    <vxe-toolbar>
      <template #buttons>
        <vxe-button @click="dd">新增</vxe-button>
        <vxe-button>
          <template #default>下拉按钮</template>
          <template #dropdowns>
            <vxe-button>删除</vxe-button>
            <vxe-button>保存</vxe-button>
          </template>
        </vxe-button>
      </template>
      <template #tools>
        <vxe-input v-model="value2" placeholder="搜索"></vxe-input>
      </template>
    </vxe-toolbar>

    <vxe-table
        border
        show-footer
        ref="tableRef"
        :column-config="{resizable: true}"
        :footer-method="footerMethod"
        :data="tableData"
        @checkbox-change="checkboxChangeEvent"
        @checkbox-all="checkboxChangeEvent">
      <vxe-column field="address" title="Address" show-overflow>
        <template #default="{ row, rowIndex }">
            <vxe-select v-model="row.flag1" transfer>
              <vxe-option v-for="num in m3" :key="num" :value="num.role" :label=num.role></vxe-option>
            </vxe-select>
        </template>
      </vxe-column>
      <vxe-column field="name" title="name" ></vxe-column>
      <vxe-column field="role" title="role" ></vxe-column>
      <vxe-column field="sex" title="sex" ></vxe-column>
      <vxe-column field="age" title="age" ></vxe-column>
    </vxe-table>
  </div>
</template>

<script lang="ts" setup>
import { reactive, ref } from 'vue'
import { VXETable, VxeTableInstance, VxeColumnPropTypes, VxeTablePropTypes } from 'vxe-table'
import XEUtils from 'xe-utils'

interface RowVO {
  id: number
  name: string
  role: string
  sex: string
  age: number
  flag1:number
  address: string
  flag:boolean
  time:number
  html1:string
  img1: string
}
const dd=()=>{
    console.log(tableData)
}
const tableRef = ref<VxeTableInstance<RowVO>>()

const tableData = ref<RowVO[]>([
  { id: 10001, name: 'Test1', role: 'Develop', flag1:6,sex: 'Man', age: 28, address: 'test abc', flag: false, time: 1600261774531, html1: '<span style="color:red">自定义HTML</span>', img1: 'https://5b0988e595225.cdn.sohucs.com/images/20181014/dce7cdaa130440e8b609fad083877ef3.gif' },
  { id: 10002, name: 'Test2', role: 'Test', flag1:6,sex: 'Women', age: 22, address: 'Guangzhou', flag: false, time: 1600261774531, html1: '', img1: 'https://5b0988e595225.cdn.sohucs.com/images/20181014/dce7cdaa130440e8b609fad083877ef3.gif' },
  { id: 10003, name: 'Test3', role: 'PM', flag1:6,sex: 'Man', age: 32, address: 'Shanghai', flag: true, time: 1600261774531, html1: '<span style="color:orange">自定义HTML</span>', img1: 'https://pic2.zhimg.com/50/v2-f7031359103859e1ed38559715ef5f3f_hd.gif' },
  { id: 10004, name: 'Test4', role: 'Designer',flag1:6, sex: 'Women', age: 23, address: 'test abc', flag: false, time: 1600261774531, html1: '', img1: 'https://pic2.zhimg.com/50/v2-f7031359103859e1ed38559715ef5f3f_hd.gif' },
  { id: 10005, name: 'Test5', role: 'Develop', flag1:6,sex: 'Women', age: 30, address: 'Shanghai', flag: true, time: 1600261774531, html1: '', img1: 'https://5b0988e595225.cdn.sohucs.com/images/20181014/dce7cdaa130440e8b609fad083877ef3.gif' },
  { id: 10006, name: 'Test6', role: 'Designer', flag1:6,sex: 'Women', age: 21, address: 'test abc', flag: true, time: 1600261774531, html1: '<span style="color:blue">自定义HTML</span>', img1: 'https://pic2.zhimg.com/50/v2-f7031359103859e1ed38559715ef5f3f_hd.gif' },
  { id: 10007, name: 'Test7', role: 'Test', flag1:6,sex: 'Man', age: 29, address: 'test abc', flag: false, time: 1600261774531, html1: '', img1: 'https://5b0988e595225.cdn.sohucs.com/images/20181014/dce7cdaa130440e8b609fad083877ef3.gif' },
  { id: 10008, name: 'Test8', role: 'Develop', flag1:6,sex: 'Man', age: 35, address: 'test abc', flag: false, time: 1600261774531, html1: '', img1: 'https://5b0988e595225.cdn.sohucs.com/images/20181014/dce7cdaa130440e8b609fad083877ef3.gif' }
])

const m3 =ref([
  { id: 10001, name: 'Test1', role:11},
  { id: 10003, name: 'Test3', role:22},
  { id: 10004, name: 'Test4', role:33},
  { id: 10005, name: 'Test5', role:44},
  { id: 10006, name: 'Test6', role:55},
])
const selectRow = ref<RowVO>()
const selectRecords = ref<RowVO[]>([])
const isAllChecked = ref(false)
const isIndeterminate = ref(false)
const showDetails = ref(false)

const value1 = ref('')
const value2 = ref('')



const sexOptions = ref([
  { data: '' }
])

const formatDate = (value: any) => {
  return XEUtils.toDateString(value, 'yyyy-MM-dd HH:mm:ss.S')
}

const filterSexMethod: VxeColumnPropTypes.FilterMethod = ({ option, row }) => {
  return row.sex === option.data
}

const changeFilterEvent = (event: any, option: any, $panel: any) => {
  $panel.changeOption(event, !!option.data, option)
}

const showDetailEvent = (row: RowVO) => {
  selectRow.value = row
  showDetails.value = true
}

const clickFooterItem = (items: any, _columnIndex: any) => {
  VXETable.modal.alert(`点击了表尾第${_columnIndex}列`)
}

const checkboxChangeEvent = () => {
  const $table = tableRef.value
  if ($table) {
    isAllChecked.value = $table.isAllCheckboxChecked()
    isIndeterminate.value = $table.isAllCheckboxIndeterminate()
    selectRecords.value = $table.getCheckboxRecords()
  }
}

const changeAllEvent = () => {
  const $table = tableRef.value
  if ($table) {
    $table.setAllCheckboxRow(isAllChecked.value)
    selectRecords.value = $table.getCheckboxRecords()
  }
}

const sumNum = (list: RowVO[], field: string) => {
  let count = 0
  list.forEach(item => {
    count += Number(item[field])
  })
  return count
}

const footerMethod: VxeTablePropTypes.FooterMethod<RowVO> = ({ columns, data }) => {
  return [
    columns.map(column => {
      if (['sex', 'num'].includes(column.field)) {
        return sumNum(data, column.field)
      }
      return null
    })
  ]
}
</script>
