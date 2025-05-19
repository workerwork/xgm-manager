<template>
  <div>
    <h1>Test Component</h1>
    <button @click="toggleExpand">点击展开</button>
    <p v-if="expanded">这里是展开的内容</p>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'Test',
  setup() {
    const expanded = ref(false);

    const toggleExpand = () => {
      expanded.value = !expanded.value;
    };

    return {
      expanded,
      toggleExpand,
    };
  },
});
</script>
