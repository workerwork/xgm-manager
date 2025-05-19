<template>
    <div class="notebook-container">
      <iframe
        src="http://127.0.0.1:8888"
        frameborder="0"
        class="notebook-iframe"
      ></iframe>
    </div>
  </template>
  
  <script setup lang="ts">
  // 你也可以用 onMounted 来验证 token 并附带参数等
  </script>
  
  <style scoped>
.notebook-container {
  width: 100%;
  height: 100%;
  margin: 0; /* 去除容器的外边距 */
  padding: 0; /* 去除容器的内边距 */
  overflow: hidden; /* 确保没有滚动条 */
  box-sizing: border-box; /* 包括内外边距在内计算元素的尺寸 */
}

.notebook-iframe {
  width: 100%;
  height: 100%;
  border: none;
  margin: 0; /* 去除 iframe 的外边距 */
  padding: 0; /* 去除 iframe 的内边距 */
  display: block; /* 确保 iframe 不会显示任何默认的内外边距 */
}
  </style>
  