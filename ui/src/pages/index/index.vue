<template>
  <view class="content">
    <view class="input-content">
      <view class="row">
        <label>时间：</label>
        <view class="item">
          <input class="input" v-model.number="year" id="year" type="number" />
          <label for="year">年</label>
        </view>

        <view class="item">
          <input
            class="small-input"
            id="month"
            type="number"
            v-model.number="month"
          />
          <label for="month">月</label>
        </view>

        <view class="item">
          <input
            class="small-input"
            id="day"
            type="number"
            v-model.number="day"
          />
          <label for="day">日</label>
        </view>

        <view class="item">
          <input
            class="small-input"
            id="hour"
            type="number"
            v-model.number="hour"
          />
          <label for="hour">时</label>
        </view>

        <view class="item">
          <input
            class="small-input"
            id="minute"
            type="number"
            v-model.number="minute"
          />
          <label for="minute">分</label>
        </view>

        <view class="item">
          <input
            class="small-input"
            id="second"
            type="number"
            v-model.number="second"
          />
          <label for="second">秒</label>
        </view>
      </view>

      <view class="row">
        <view class="item">
          <text>性别：</text>
          <radio-group @change="sexChange">
            <radio value="male" :checked="sex == 'male'" />
            <text>男</text>
            <radio value="female" :checked="sex == 'female'" />
            <text>女</text>
          </radio-group>
        </view>
      </view>

      <!-- 流年 -->
      <view class="row">
        <label>流年：</label>
        <view class="item">
          <input
            class="input"
            v-model.number="process_year"
            id="process_year"
            type="number"
          />
          <label for="year">年</label>
        </view>

        <view class="item">
          <input
            class="small-input"
            id="process_month"
            type="number"
            v-model.number="process_month"
          />
          <label for="month">月</label>
        </view>

        <view class="item">
          <input
            class="small-input"
            id="process_day"
            type="number"
            v-model.number="process_day"
          />
          <label for="day">日</label>
        </view>

        <view class="item">
          <input
            class="small-input"
            id="process_hour"
            type="number"
            v-model.number="process_hour"
          />
          <label for="hour">时</label>
        </view>

        <view class="item">
          <input
            class="small-input"
            id="process_minute"
            type="number"
            v-model.number="process_minute"
          />
          <label for="minute">分</label>
        </view>

        <view class="item">
          <input
            class="small-input"
            id="process_second"
            type="number"
            v-model.number="process_second"
          />
          <label for="second">秒</label>
        </view>
      </view>

      <navigator
        :url="
          '/pages/ziwei/index?item=' +
          encodeURIComponent(JSON.stringify(requestData))
        "
        open-type="navigate"
        hover-class="navigator-hover"
      >
        <button type="primary" :loading="false" hover-class="button-hover">
          提交
        </button>
      </navigator>
    </view>
  </view>
</template>

<script setup lang="ts">
import type { RequestData } from "@/type";
import { nowDateTime } from "@/utils";
import { computed, ref } from "vue";

const sex = ref("male");

const t = nowDateTime();
const year = ref(t.year);
const month = ref(t.month);
const day = ref(t.day);
const hour = ref(t.hour);
const minute = ref(t.minute);
const second = ref(t.second);

const process_year = ref(t.year);
const process_month = ref(t.month);
const process_day = ref(t.day);
const process_hour = ref(t.hour);
const process_minute = ref(t.minute);
const process_second = ref(t.second);

const requestData = computed<RequestData>(() => {
  return {
    year: year.value,
    month: month.value,
    day: day.value,
    hour: hour.value,
    minute: minute.value,
    second: second.value,
    masculine: sex.value == "male",
    process_year: process_year.value,
    process_month: process_month.value,
    process_day: process_day.value,
    process_hour: process_hour.value,
    process_minute: process_minute.value,
    process_second: process_second.value,
  };
});

function sexChange(event: { detail: { value: string } }) {
  sex.value = event.detail.value;
}
</script>

<style lang="scss" scoped>
.content {
  display: flex;
  flex-direction: row;
  justify-content: center;
}

// .input-content {
//   display: flex;
//   flex-direction: column;
// }

.row {
  display: flex;
  margin-top: 20rpx;
  margin-bottom: 20rpx;
  .item {
    display: flex;
    margin-right: 25rpx;
  }
}

.input {
  background: #fff;
  width: 80rpx;
}
.small-input {
  width: 40rpx;
}
</style>
