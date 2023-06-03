<template>
  <view class="content">
    <view>
      <view class="row top-border left-border right-border">
        <House
          class="box right-border"
          :name="pan.data.houses[3]"
          :gan-zhi="pan.data.gong_gan[3] + '巳'"
          :stars="pan.data.stars[3]"
        ></House>
        <House
          class="box right-border"
          :name="pan.data.houses[4]"
          :gan-zhi="pan.data.gong_gan[4] + '午'"
          :stars="pan.data.stars[4]"
        ></House>

        <House
          class="box right-border"
          :name="pan.data.houses[5]"
          :gan-zhi="pan.data.gong_gan[5] + '未'"
          :stars="pan.data.stars[5]"
        ></House>

        <House
          class="box"
          :name="pan.data.houses[6]"
          :gan-zhi="pan.data.gong_gan[6] + '申'"
          :stars="pan.data.stars[6]"
        ></House>
      </view>
      <view style="display: flex" class="top-border left-border right-border">
        <vie class="right-border">
          <House
            class="box"
            :name="pan.data.houses[2]"
            :gan-zhi="pan.data.gong_gan[2] + '辰'"
            :stars="pan.data.stars[2]"
          ></House>

          <House
            class="box top-border"
            :name="pan.data.houses[1]"
            :gan-zhi="pan.data.gong_gan[1] + '卯'"
            :stars="pan.data.stars[1]"
          ></House>
        </vie>
        <!-- 信息框 -->
        <vie class="info-box">
          <Info :request-data="requestData" :pan="pan.data"></Info>
        </vie>
        <!-- house -->
        <vie class="left-border">
          <House
            class="box"
            :name="pan.data.houses[7]"
            :gan-zhi="pan.data.gong_gan[7] + '酉'"
            :stars="pan.data.stars[7]"
          ></House>

          <House
            class="box top-border"
            :name="pan.data.houses[8]"
            :gan-zhi="pan.data.gong_gan[8] + '戌'"
            :stars="pan.data.stars[8]"
          ></House>
        </vie>
      </view>
      <view class="row top-border bottom-border left-border right-border">
        <House
          class="box right-border"
          :name="pan.data.houses[0]"
          :gan-zhi="pan.data.gong_gan[0] + '寅'"
          :stars="pan.data.stars[0]"
        ></House>

        <House
          class="box right-border"
          :name="pan.data.houses[11]"
          :gan-zhi="pan.data.gong_gan[11] + '丑'"
          :stars="pan.data.stars[11]"
        ></House>

        <House
          class="box right-border"
          :name="pan.data.houses[10]"
          :gan-zhi="pan.data.gong_gan[10] + '子'"
          :stars="pan.data.stars[10]"
        ></House>

        <House
          class="box"
          :name="pan.data.houses[9]"
          :gan-zhi="pan.data.gong_gan[9] + '亥'"
          :stars="pan.data.stars[9]"
        ></House>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { api } from "@/http";
import type { RequestData, ZiWeiPan } from "@/type";
import { onLoad } from "@dcloudio/uni-app";
import { reactive } from "vue";

import House from "@/components/house.vue";
import Info from "@/components/info.vue";

const requestData: RequestData = reactive({
  year: 0,
  month: 1,
  day: 1,
  hour: 0,
  minute: 0,
  second: 0,
  masculine: true,
  process_year: 0,
  // 推运月
  process_month: 0,
  // 推运日
  process_day: 0,
  // 推运时
  process_hour: 0,
  // 推运分
  process_minute: 0,
  // 推运秒
  process_second: 0,
});

const ziWeiPan: ZiWeiPan = {
  lunar_calendar: {
    year: "",
    month: "",
    day: "",
    time_gan_zhi: "",
  },
  solar_term_first: {
    name: "",
    year: 0,
    month: 0,
    day: 0,
    hour: 0,
    minute: 0,
    second: 0,
  },
  solar_term_second: {
    name: "",
    year: 0,
    month: 0,
    day: 0,
    hour: 0,
    minute: 0,
    second: 0,
  },
  si_zhu: {
    year: "",
    month: "",
    day: "",
    time: "",
  },
  kong: [],
  gong_gan: [],
  houses: [],
  shen_gong: "",
  wu_xing_num: 0,
  stars: [],
  lord: "",
  shen_lord: "",
  primary_process: "",
  second_process: "",
  dou_jun: "",
};
const pan = reactive({
  data: ziWeiPan,
  completed: false,
  //   err: false,
  //   errMessage: "",
});

onLoad((option: any) => {
  const request = JSON.parse(decodeURIComponent(option.item));
  Object.assign(requestData, request);
  uni
    .request({
      url: api.url,
      method: api.method,
      header: api.header,
      data: request,
    })
    .then((res: any) => {
      pan.data = res.data;
      pan.completed = true;
    });
});
</script>

<style lang="scss" scoped>
.content {
  display: flex;
  flex-direction: row;
  justify-content: center;
  margin: 20rpx;
}
.row {
  display: flex;
}

$boxWidth: 300rpx;
$boxheight: 500rpx;

.box {
  display: flex;
  flex-direction: column;
  width: $boxWidth;
  height: $boxheight;
}

.info-box {
  display: flex;
  flex-direction: row;
  justify-content: center;
  padding-top: 10rpx;
  width: 2 * $boxWidth;
  height: 2 * $boxheight;
}

$boxBorder: 2rpx;
$boxBorderColor: black;
.top-border {
  border-top: $boxBorder solid $boxBorderColor;
}

.bottom-border {
  border-bottom: $boxBorder solid $boxBorderColor;
}

.left-border {
  border-left: $boxBorder solid $boxBorderColor;
}
.right-border {
  border-right: $boxBorder solid $boxBorderColor;
}
</style>
