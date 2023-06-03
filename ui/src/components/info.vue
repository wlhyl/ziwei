<template>
  <view>
    <!-- 出生时间 -->
    <view class="item">
      <text>
        时间：{{ requestData.year }}年{{ requestData.month }}月{{
          requestData.day
        }}日{{ requestData.hour }}:{{ requestData.minute }}:{{
          requestData.second
        }}
      </text>
    </view>
    <!-- 农历 -->
    <view class="item">
      <text>
        农历：{{ pan.lunar_calendar.year }}年{{ pan.lunar_calendar.month
        }}{{ pan.lunar_calendar.day }} {{ pan.lunar_calendar.time_gan_zhi }}时
      </text>
    </view>
    <!-- 四柱 -->
    <view class="item">
      <text>
        干支：{{ pan.si_zhu.year }}年 {{ pan.si_zhu.month }}月
        {{ pan.si_zhu.day }}日 {{ pan.si_zhu.time }}时
      </text>
    </view>

    <!-- 节气 -->
    <view class="item" style="display: flex">
      <text>节气</text>：
      <view style="display: flex; flex-direction: column">
        <view>
          <text>
            {{ pan.solar_term_first.name }}: {{ pan.solar_term_first.year }}-{{
              pan.solar_term_first.month
            }}-{{ pan.solar_term_first.day }} {{ pan.solar_term_first.hour }}:{{
              pan.solar_term_first.minute
            }}:{{ pan.solar_term_first.second }}
          </text>
        </view>
        <view>
          <text>
            {{ pan.solar_term_second.name }}:
            {{ pan.solar_term_second.year }}-{{
              pan.solar_term_second.month
            }}-{{ pan.solar_term_second.day }}
            {{ pan.solar_term_second.hour }}:{{
              pan.solar_term_second.minute
            }}:{{ pan.solar_term_second.second }}
          </text>
        </view>
      </view>
    </view>

    <!-- 五行局 -->
    <view>
      <text>{{ masculine }}：{{ wuXingNum }}</text>
    </view>
    <!-- 命主、身主、身宫 -->
    <view>
      <text style="margin-right: 20rpx">命主：{{ pan.lord }}</text>
      <text style="margin-right: 20rpx">身主：{{ pan.shen_lord }}</text>
      <text>身宫：{{ pan.shen_gong }}</text>
    </view>
    <!-- 流年 -->
    <view class="item">
      <text>
        流年：{{ requestData.process_year }}年{{
          requestData.process_month
        }}月{{ requestData.process_day }}日{{ requestData.process_hour }}:{{
          requestData.process_minute
        }}:{{ requestData.process_second }}
      </text>
    </view>
    <!-- 大运、小限、斗君 -->
    <view>
      <text style="margin-right: 20rpx">大运：{{ pan.primary_process }} </text>
      <text style="margin-right: 20rpx">小限：{{ pan.second_process }} </text>
      <text>斗君：{{ pan.dou_jun }}</text>
    </view>
  </view>
</template>
<script setup lang="ts">
import type { RequestData, ZiWeiPan } from "@/type";
import { computed } from "vue";

interface InfoProps {
  requestData: RequestData;
  pan: ZiWeiPan;
}
const props = defineProps<InfoProps>();

// 命局阴阳
const masculine = computed(() => {
  let sex = props.requestData.masculine ? "男" : "女";

  let s = ["甲", "丙", "戊", "庚", "壬"].includes(props.pan.si_zhu.year[0])
    ? "阳"
    : "阴";
  return `${s}${sex}`;
});

// 五行局
const wuXingNum = computed(() => {
  switch (props.pan.wu_xing_num) {
    case 2:
      return "水二局";
    case 3:
      return "木三局";
    case 4:
      return "金四局";
    case 5:
      return "土五局";
    default:
      return "火六局";
  }
});
</script>
<style lang="scss" scoped></style>