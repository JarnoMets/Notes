<template>
  <div class="boards-view">
    <div class="header">
      <h1>Boards</h1>
      <button class="btn btn-primary" @click="showCreateModal = true">+ New Board</button>
    </div>

    <div class="boards-grid" v-if="boards.length > 0">
      <div 
        v-for="board in boards" 
        :key="board.id" 
        class="board-card"
        :style="{ borderTopColor: board.color || '#3498db' }"
        @click="router.push(`/boards/${board.id}`)"
      >
        <h3>{{ board.name }}</h3>
        <p v-if="board.description" class="board-description">{{ board.description }}</p>
        <div class="board-meta">
          <span>{{ formatDate(board.updated_at) }}</span>
        </div>
      </div>
    </div>

    <div v-else class="empty-state">
      <p>No boards yet. Create your first board to start planning!</p>
    </div>

    <!-- Create Board Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click.self="showCreateModal = false">
      <div class="modal">
        <div class="modal-header">
          <h2>New Board</h2>
          <button class="modal-close" @click="showCreateModal = false">&times;</button>
        </div>
        <form @submit.prevent="createBoard">
          <div class="form-group">
            <label for="name">Name</label>
            <input 
              id="name" 
              v-model="newBoard.name" 
              type="text" 
              required 
              placeholder="Board name"
            />
          </div>
          <div class="form-group">
            <label for="description">Description (optional)</label>
            <textarea 
              id="description" 
              v-model="newBoard.description" 
              rows="3" 
              placeholder="What's this board for?"
            ></textarea>
          </div>
          <div class="form-group">
            <label for="color">Color</label>
            <div class="color-picker">
              <button 
                v-for="color in colors" 
                :key="color"
                type="button"
                class="color-option"
                :class="{ selected: newBoard.color === color }"
                :style="{ backgroundColor: color }"
                @click="newBoard.color = color"
              ></button>
            </div>
          </div>
          <div class="modal-actions">
            <button type="button" class="btn btn-secondary" @click="showCreateModal = false">Cancel</button>
            <button type="submit" class="btn btn-primary">Create</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { boardsApi } from '../api'
import { formatDate } from '../composables'
import type { Board } from '../types'

const router = useRouter()
const boards = ref<Board[]>([])
const showCreateModal = ref(false)
const newBoard = ref({ name: '', description: '', color: '#3498db' })

const colors = [
  '#3498db', '#2ecc71', '#e74c3c', '#f39c12', 
  '#9b59b6', '#1abc9c', '#e67e22', '#34495e'
]

const fetchBoards = async () => {
  try {
    const response = await boardsApi.getAll()
    boards.value = response.data
  } catch (error) {
    console.error('Failed to fetch boards:', error)
  }
}

const createBoard = async () => {
  try {
    await boardsApi.create(newBoard.value)
    showCreateModal.value = false
    newBoard.value = { name: '', description: '', color: '#3498db' }
    await fetchBoards()
  } catch (error) {
    console.error('Failed to create board:', error)
  }
}

onMounted(fetchBoards)
</script>

<style scoped>
.boards-view {
  max-width: 1200px;
  margin: 0 auto;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.boards-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1.5rem;
}

.board-card {
  background: white;
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
  border-top: 4px solid;
}

.board-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
}

.board-card h3 {
  margin-bottom: 0.5rem;
  color: #2c3e50;
}

.board-description {
  color: #666;
  font-size: 0.9rem;
  margin-bottom: 1rem;
}

.board-meta {
  font-size: 0.8rem;
  color: #999;
}

.empty-state {
  text-align: center;
  padding: 4rem;
  color: #666;
}

.color-picker {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.color-option {
  width: 32px;
  height: 32px;
  border-radius: 4px;
  border: 2px solid transparent;
  cursor: pointer;
  transition: transform 0.2s;
}

.color-option:hover {
  transform: scale(1.1);
}

.color-option.selected {
  border-color: #2c3e50;
}
</style>
