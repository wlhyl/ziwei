<template>
  <view>
    <view class="star">
      <div class="item" v-for="star in stars">
        <text :class="star.category">
          {{ star.name }}
        </text>
        <text v-if="star.wu_xing">({{ star.wu_xing }})</text>
        <text v-if="star.power">{{ star.power }}</text>
        <text class="v_star" v-if="star.v_star">
          {{ star.v_star }}
        </text>
        <text v-if="star.lord">[{{ star.lord }}]</text>
      </div>
    </view>
    <view class="house">
      <text class="name">{{ name }}</text>
      <text class="gan">{{ ganZhi }}</text>
    </view>
  </view>
</template>
<script setup lang="ts">
import type { Star } from "@/type";

interface HouseProps {
  name: string;
  ganZhi: string;
  stars: Array<Star>;
}
const props = defineProps<HouseProps>();
</script>
<style lang="scss" scoped>
.star {
  display: flex;
  flex-grow: 1;
  .item {
    // height: 400rpx;
    // width: 50rpx;
    writing-mode: vertical-lr;
    margin-right: 2rpx;
    padding: 0rpx;
    // overflow: hidden;
    // &:hover{
    //   overflow: visible;
    // }
  }
}

.N {
  color: #1f45fc;
}

.S {
  color: red;
}

//   吉星
.Benefic {
  color: green;
  font-weight: bold;
}
/// 凶星
.Malefics {
  color: #ffa62f;
  font-weight: bold;
}

// 禄存，天马
.Money {
  color: #966f33;
}

.v_star {
  border-radius: 5rpx;
  padding: 0rpx;
  background-color: red;
  color: white;
}

// 普通星
.common {
  color: black;
}

.house {
  display: flex;
  align-items: flex-end;
  justify-content: flex-end;
  font-weight: bold;
  // 宫位名
  .name {
    margin: 10rpx;
    color: red;
  }
  // 宫位干支
  .gan {
    writing-mode: vertical-lr;
  }
}
</style>