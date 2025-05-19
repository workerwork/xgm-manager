<template>
  <div v-if="show" class="modal-overlay" @click.self="close">
    <div class="modal-content">
      <div class="modal-header">
        <h2>包详情：{{ pkg[0] }}</h2>
        <button class="close-button" @click="close">×</button>
      </div>

      <div class="modal-body">
        <p><strong>名称：</strong>{{ pkg[0] }}</p>
        <p><strong>版本：</strong>{{ pkg[1] }}</p>
        <p><strong>位置：</strong>{{ pkg[2] }}</p>
        <p><strong>依赖：</strong>{{ pkg[3] || '无依赖' }}</p>
      </div>

      <div class="modal-footer">
        <button class="action-button" @click="close">关闭</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  show: boolean;
  pkg: [string, string, string, string];
}>();

const emit = defineEmits(['close']);
const close = () => emit('close');
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 10000;
}

.modal-content {
  background: white;
  border-radius: 8px;
  width: 100%;  /* Adjust the width percentage here */
  max-width: 800px; /* Increased max-width */
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 5px 15px rgba(0,0,0,0.3);
  overflow: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 30px 10px;
  border-bottom: 1px solid #eee;
}

.modal-header h2 {
  margin: 0;
  font-size: 20px;
}

.modal-body {
  padding: 10px 30px;
  overflow-y: auto;
  flex: 1;
}

.modal-footer {
  padding: 15px 30px;
  border-top: 1px solid #eee;
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.action-button {
  padding: 6px 14px;
  border: 1px solid #007bff;
  background-color: #007bff;
  color: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.action-button:hover {
  background-color: #0056b3;
}

.close-button {
  font-size: 20px;
  font-weight: bold;
  border: none;
  background: none;
  cursor: pointer;
  color: #555;
}

.close-button:hover {
  color: #f56c6c;
}
</style>
