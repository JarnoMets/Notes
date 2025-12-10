<template>
  <div class="boards-workspace">
    <!-- Mobile Sidebar Toggle -->
    <MobileSidebarToggle
      :is-open="isMobileSidebarOpen"
      @toggle="toggleMobileSidebar"
    />

    <!-- Mobile Sidebar Overlay -->
    <MobileOverlay
      :is-open="isMobileSidebarOpen"
      @close="isMobileSidebarOpen = false"
    />

    <!-- Sidebar -->
    <WorkspaceSidebar
      :width="sidebarWidth"
      :is-open="isMobileSidebarOpen"
      @refresh="refreshTree"
    >
      <ExplorerTree
        default-tab="boards"
        @select="handleExplorerSelect"
        @create-note="handleCreateNote"
        @create-folder="handleCreateFolder"
        @create-board="openCreateBoardModal"
        @create-board-folder="handleCreateBoardFolder"
        @rename="handleRename"
        @delete="handleExplorerDelete"
        @open-note="handleOpenNote"
        @open-board="selectBoard"
        @drop="handleDrop"
      />
    </WorkspaceSidebar>

    <!-- Resize Handle -->
    <ResizeHandle
      direction="vertical"
      @resize-start="startSidebarResize"
    />

    <!-- Main Content -->
    <div class="board-content"
      @dragenter="onBoardContentDragEnter"
      @dragover="onBoardContentDragOver"
      @dragleave="onBoardContentDragLeave"
      @drop="onBoardContentDrop"
      :class="{ 'drag-over': boardContentDragOver }"
    >
      <!-- Board Header -->
      <div class="board-header" v-if="currentBoard">
        <div class="header-left">
          <h2 :style="{ color: currentBoard.board.color || 'var(--text-primary)' }">
            {{ currentBoard.board.name }}
          </h2>
          <span class="board-description" v-if="currentBoard.board.description">
            {{ currentBoard.board.description }}
          </span>
        </div>
        <div class="header-actions">
          <button class="btn-icon" :class="{ active: hasActiveFilters }" @click="showFiltersPanel = !showFiltersPanel" title="Filters">
            <Icon name="filter" :size="16" />
          </button>
          <button class="btn-icon" @click="showLabelsPanel = true" title="Labels">
            <Icon name="tag" :size="16" />
          </button>
          <button class="btn-icon" @click="showAutomationsPanel = true" title="Automations">
            <Icon name="play" :size="16" />
          </button>
          <button class="btn-icon" @click="openArchivePanel" title="Archive">
            <Icon name="archive" :size="16" />
          </button>
          <button class="btn-icon" @click="openEditBoardModal" title="Edit Board">
            <Icon name="edit" :size="16" />
          </button>
          <div class="menu-wrapper" ref="menuWrapper">
            <button class="btn-icon" @click="toggleBoardMenu" title="More options">
              <Icon name="more-vertical" :size="16" :fill="true" />
            </button>
            <div v-if="showBoardMenu" class="dropdown-menu">
              <button @click="openAddListModal">
                <Icon name="plus" :size="14" /> Add List
              </button>
              <div class="menu-divider"></div>
              <button class="danger" @click="confirmDeleteBoard">
                <Icon name="trash" :size="14" /> Delete Board
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Filters Bar -->
      <div v-if="showFiltersPanel && currentBoard" class="filters-bar">
        <div class="filter-group">
          <label>Labels:</label>
          <div class="filter-labels">
            <button 
              v-for="label in currentBoard.labels" 
              :key="label.id" 
              class="filter-label"
              :class="{ selected: selectedLabelFilters.includes(label.id) }"
              :style="{ backgroundColor: selectedLabelFilters.includes(label.id) ? label.color : 'transparent', borderColor: label.color }"
              @click="toggleLabelFilter(label.id)"
            >
              {{ label.name }}
            </button>
          </div>
        </div>
        <div class="filter-group">
          <label>Due Date:</label>
          <select v-model="dueDateFilter">
            <option value="">All</option>
            <option value="overdue">Overdue</option>
            <option value="today">Due Today</option>
            <option value="week">Due This Week</option>
            <option value="none">No Due Date</option>
          </select>
        </div>
        <button class="btn btn-secondary btn-sm" @click="clearFilters">Clear Filters</button>
      </div>

      <!-- Kanban Board -->
      <div class="kanban-container" v-if="currentBoard">
        <div class="lists-wrapper">
          <div 
            v-for="listWithCards in filteredLists" 
            :key="listWithCards.list.id" 
            class="kanban-list"
            @dragover.prevent="onListDragOver($event, listWithCards.list.id)"
            @drop="onCardDrop($event, listWithCards.list.id)"
          >
            <div class="list-header" @contextmenu.prevent="openListContextMenu($event, listWithCards.list)">
              <h3>{{ listWithCards.list.name }}</h3>
              <span class="card-count">{{ getFilteredCards(listWithCards.cards).length }}</span>
              <button class="list-menu-btn" @click="archiveList(listWithCards.list.id)" title="Archive List">
                <Icon name="archive" :size="14" />
              </button>
              <button class="list-menu-btn" @click="confirmDeleteList(listWithCards.list.id)" title="Delete List">
                <Icon name="x" :size="14" />
              </button>
            </div>
            
            <div class="cards-container">
              <div 
                v-for="(card, index) in getFilteredCards(listWithCards.cards)" 
                :key="card.id" 
                class="kanban-card"
                :class="{ 
                  'drag-over': dragOverCard === card.id,
                  'note-drop-target': noteDropTargetCardId === card.id
                }"
                draggable="true"
                @dragstart="onCardDragStart($event, card, listWithCards.list.id)"
                @dragend="onCardDragEnd"
                @dragover.prevent="onCardDragOver($event, card.id, index, listWithCards.list.id)"
                @dragleave="onCardDragLeave"
                @drop.stop="onCardDropOnCard($event, card.id, index, listWithCards.list.id)"
                @click="openCardModal(card)"
                @contextmenu.prevent="openCardContextMenu($event, card, listWithCards.list.id)"
              >
                <!-- Card Edit Button -->
                <button 
                  class="card-edit-btn" 
                  @click.stop="openCardModal(card)" 
                  title="Edit card"
                >
                  <Icon name="edit" :size="12" />
                </button>
                
                <!-- Note link indicator -->
                <div v-if="hasNoteLinks(card.description)" class="card-links">
                  <span class="link-indicator" title="Has linked notes">
                    <Icon name="link" :size="12" />
                  </span>
                </div>
                <div class="card-labels" v-if="card.labels.length > 0">
                  <span 
                    v-for="labelId in card.labels" 
                    :key="labelId" 
                    class="label"
                    :style="{ backgroundColor: getLabelColor(labelId) }"
                  >{{ getLabelName(labelId) }}</span>
                </div>
                <h4>{{ card.title }}</h4>
                <p v-if="card.description" class="card-description">{{ getPreview(card.description) }}</p>
                <div class="card-footer" v-if="card.due_date">
                  <span class="due-date" :class="{ overdue: isOverdue(card.due_date) }">
                    <Icon name="calendar" :size="12" /> {{ formatDate(card.due_date) }}
                  </span>
                </div>
              </div>
              
              <!-- Drop zone at end of list -->
              <div 
                class="card-drop-zone"
                :class="{ 'active': dropZoneListId === listWithCards.list.id }"
                @dragover.prevent
                @drop="onCardDropAtEnd($event, listWithCards.list.id)"
              ></div>
            </div>

            <button class="add-card-btn" @click="openAddCardModal(listWithCards.list.id)">
              <Icon name="plus" :size="14" /> Add Card
            </button>
          </div>

          <!-- Add List Button -->
          <div class="kanban-list add-list-placeholder" @click="openAddListModal">
            <Icon name="plus" :size="16" />
            <span>Add List</span>
          </div>
        </div>
      </div>

      <!-- Empty State -->
      <div class="board-empty" v-else>
        <div class="empty-content">
          <Icon name="board" :size="48" />
          <h3>No Board Selected</h3>
          <p>Select a board from the sidebar or create a new one</p>
          <button class="btn btn-primary" @click="openCreateBoardModal">
            <Icon name="plus" :size="16" /> Create Board
          </button>
        </div>
      </div>
    </div>

    <!-- Context Menu -->
    <div 
      v-if="contextMenu.visible"
      class="context-menu"
      :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }"
      @click.stop
    >
      <div class="context-menu-item" @click="handleContextAction('edit')">
        <Icon name="edit" :size="14" /> Edit
      </div>
      <div class="context-menu-divider"></div>
      <div class="context-menu-item danger" @click="handleContextAction('delete')">
        <Icon name="trash" :size="14" /> Delete
      </div>
    </div>

    <!-- Create/Edit Board Modal -->
    <div v-if="boardModal.visible" class="modal-overlay" @click.self="closeBoardModal">
      <div class="modal">
        <div class="modal-header">
          <h3>{{ boardModal.isEdit ? 'Edit Board' : 'New Board' }}</h3>
          <button class="modal-close" @click="closeBoardModal">
            <Icon name="x" :size="14" />
          </button>
        </div>
        <form @submit.prevent="saveBoardModal">
          <div class="form-group">
            <label for="boardName">Name</label>
            <input 
              id="boardName" 
              v-model="boardModal.name" 
              type="text" 
              required 
              placeholder="Board name"
            />
          </div>
          <div class="form-group">
            <label for="boardDescription">Description (optional)</label>
            <textarea 
              id="boardDescription" 
              v-model="boardModal.description" 
              rows="3" 
              placeholder="What's this board for?"
            ></textarea>
          </div>
          <div class="form-group">
            <label>Color</label>
            <div class="color-picker">
              <button 
                v-for="color in colorPalette" 
                :key="color"
                type="button"
                class="color-option"
                :class="{ selected: boardModal.color === color }"
                :style="{ backgroundColor: color }"
                @click="boardModal.color = color"
              ></button>
            </div>
          </div>
          <div class="modal-actions">
            <button type="button" class="btn btn-secondary" @click="closeBoardModal">Cancel</button>
            <button type="submit" class="btn btn-primary">{{ boardModal.isEdit ? 'Save' : 'Create' }}</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Add List Modal -->
    <div v-if="showAddListModal" class="modal-overlay" @click.self="showAddListModal = false">
      <div class="modal">
        <div class="modal-header">
          <h3>Add List</h3>
          <button class="modal-close" @click="showAddListModal = false">
            <Icon name="x" :size="14" />
          </button>
        </div>
        <form @submit.prevent="addList">
          <div class="form-group">
            <label for="listName">List Name</label>
            <input 
              id="listName" 
              v-model="newListName" 
              type="text" 
              required 
              placeholder="e.g., To Do, In Progress, Done"
            />
          </div>
          <div class="modal-actions">
            <button type="button" class="btn btn-secondary" @click="showAddListModal = false">Cancel</button>
            <button type="submit" class="btn btn-primary">Add List</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Add Card Modal -->
    <div v-if="showAddCardModal" class="modal-overlay" @click.self="showAddCardModal = false">
      <div class="modal">
        <div class="modal-header">
          <h3>Add Card</h3>
          <button class="modal-close" @click="showAddCardModal = false">
            <Icon name="x" :size="14" />
          </button>
        </div>
        <form @submit.prevent="addCard">
          <div class="form-group">
            <label for="cardTitle">Title</label>
            <input 
              id="cardTitle" 
              v-model="newCard.title" 
              type="text" 
              required 
              placeholder="Card title"
            />
          </div>
          <div class="form-group">
            <label for="cardDescription">Description (optional)</label>
            <textarea 
              id="cardDescription" 
              v-model="newCard.description" 
              rows="3" 
              placeholder="Add more details..."
            ></textarea>
          </div>
          <div class="modal-actions">
            <button type="button" class="btn btn-secondary" @click="showAddCardModal = false">Cancel</button>
            <button type="submit" class="btn btn-primary">Add Card</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Edit Card Modal -->
    <div v-if="showEditCardModal && editingCard" class="modal-overlay" @click.self="closeEditCardModal">
      <div class="modal modal-large">
        <div class="modal-header">
          <h3>Edit Card</h3>
          <button class="modal-close" @click="closeEditCardModal">
            <Icon name="x" :size="14" />
          </button>
        </div>
        <form @submit.prevent="updateCard">
          <div class="form-group">
            <label for="editCardTitle">Title</label>
            <input 
              id="editCardTitle" 
              v-model="editingCard.title" 
              type="text" 
              required 
            />
          </div>
          <div class="form-group">
            <label for="editCardDescription">Description</label>
            <textarea 
              id="editCardDescription" 
              v-model="editingCard.description" 
              rows="6" 
              placeholder="Add a more detailed description..."
            ></textarea>
          </div>
          <div class="form-group">
            <label for="editCardDueDate">Due Date</label>
            <input 
              id="editCardDueDate" 
              v-model="editingCard.due_date" 
              type="datetime-local" 
            />
          </div>
          <div class="form-group">
            <label>Labels</label>
            <div class="label-selector">
              <button 
                v-for="label in currentBoard?.labels" 
                :key="label.id"
                type="button"
                class="label-option"
                :class="{ selected: editingCard.labels.includes(label.id) }"
                :style="{ backgroundColor: editingCard.labels.includes(label.id) ? label.color : 'transparent', borderColor: label.color }"
                @click="toggleCardLabel(label.id)"
              >
                {{ label.name }}
              </button>
            </div>
          </div>
          
          <!-- Linked Items Section -->
          <div v-if="getLinkedItems(editingCard.description).length > 0" class="form-group">
            <label>Linked Items</label>
            <div class="linked-items-list">
              <div 
                v-for="item in getLinkedItems(editingCard.description)" 
                :key="item.href"
                class="linked-item"
                @click="navigateToLinkedItem(item)"
              >
                <Icon :name="item.type === 'note' ? 'file' : 'board'" :size="14" />
                <span class="linked-item-name">{{ item.name }}</span>
                <button 
                  type="button" 
                  class="linked-item-remove" 
                  @click.stop="removeLinkedItem(item)"
                  title="Remove link"
                >
                  <Icon name="x" :size="12" />
                </button>
              </div>
            </div>
          </div>
          
          <div class="modal-actions">
            <button type="button" class="btn btn-warning" @click="archiveCard">Archive</button>
            <button type="button" class="btn btn-danger" @click="confirmDeleteCard">Delete</button>
            <button type="button" class="btn btn-secondary" @click="closeEditCardModal">Cancel</button>
            <button type="submit" class="btn btn-primary">Save Changes</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Labels Management Panel -->
    <div v-if="showLabelsPanel" class="side-panel-overlay" @click.self="showLabelsPanel = false">
      <div class="side-panel">
        <div class="side-panel-header">
          <h3>Labels</h3>
          <button class="modal-close" @click="showLabelsPanel = false">
            <Icon name="x" :size="14" />
          </button>
        </div>
        <div class="side-panel-content">
          <div class="labels-list-manage">
            <div 
              v-for="label in currentBoard?.labels" 
              :key="label.id" 
              class="label-item"
            >
              <div class="label-preview" :style="{ backgroundColor: label.color }">
                {{ label.name }}
              </div>
              <div class="label-actions">
                <button class="btn btn-sm" @click="editLabel(label)"><Icon name="edit" :size="12" /></button>
                <button class="btn btn-sm btn-danger" @click="deleteLabelById(label.id)"><Icon name="trash" :size="12" /></button>
              </div>
            </div>
          </div>
          <div class="create-label-form">
            <h4>Create Label</h4>
            <input v-model="newLabelName" placeholder="Label name" />
            <div class="color-picker">
              <button 
                v-for="color in labelColorPalette" 
                :key="color"
                type="button"
                class="color-option"
                :class="{ selected: newLabelColor === color }"
                :style="{ backgroundColor: color }"
                @click="newLabelColor = color"
              ></button>
            </div>
            <button class="btn btn-primary" @click="createLabel" :disabled="!newLabelName.trim()">Create</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Archive Panel -->
    <div v-if="showArchivePanel" class="side-panel-overlay" @click.self="showArchivePanel = false">
      <div class="side-panel">
        <div class="side-panel-header">
          <h3>Archived Items</h3>
          <button class="modal-close" @click="showArchivePanel = false">
            <Icon name="x" :size="14" />
          </button>
        </div>
        <div class="side-panel-content">
          <div class="archive-section">
            <h4>Archived Lists</h4>
            <div v-if="archivedLists.length === 0" class="empty-state">No archived lists</div>
            <div v-for="list in archivedLists" :key="list.id" class="archived-item">
              <span>{{ list.name }}</span>
              <button class="btn btn-sm" @click="restoreList(list.id)">Restore</button>
            </div>
          </div>
          <div class="archive-section">
            <h4>Archived Cards</h4>
            <div v-if="archivedCards.length === 0" class="empty-state">No archived cards</div>
            <div v-for="card in archivedCards" :key="card.id" class="archived-item">
              <span>{{ card.title }}</span>
              <button class="btn btn-sm" @click="restoreCard(card.id)">Restore</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Automations Panel -->
    <div v-if="showAutomationsPanel" class="side-panel-overlay" @click.self="showAutomationsPanel = false">
      <div class="side-panel side-panel-wide">
        <div class="side-panel-header">
          <h3>Automations</h3>
          <button class="modal-close" @click="showAutomationsPanel = false">
            <Icon name="x" :size="14" />
          </button>
        </div>
        <div class="side-panel-content">
          <div class="automations-list">
            <div 
              v-for="automation in currentBoard?.automations" 
              :key="automation.id" 
              class="automation-item"
              :class="{ disabled: !automation.enabled }"
            >
              <div class="automation-info">
                <div class="automation-name">{{ automation.name }}</div>
                <div class="automation-description">
                  When <strong>{{ getTriggerLabel(automation.trigger_type) }}</strong> 
                  → <strong>{{ getActionLabel(automation.action_type) }}</strong>
                </div>
              </div>
              <div class="automation-actions">
                <button class="btn btn-sm" @click="toggleAutomation(automation.id)">
                  {{ automation.enabled ? 'Disable' : 'Enable' }}
                </button>
                <button class="btn btn-sm btn-danger" @click="deleteAutomation(automation.id)">
                  <Icon name="trash" :size="12" />
                </button>
              </div>
            </div>
          </div>
          <div class="create-automation-form">
            <h4>Create Automation</h4>
            <div class="form-group">
              <label>Name</label>
              <input v-model="newAutomation.name" placeholder="e.g., Move to Done when complete" />
            </div>
            <div class="form-group">
              <label>When...</label>
              <select v-model="newAutomation.trigger_type">
                <option value="card_moved">Card is moved to a list</option>
                <option value="due_date_passed">Due date passes</option>
                <option value="label_added">Label is added</option>
                <option value="card_created">Card is created</option>
              </select>
              <div v-if="newAutomation.trigger_type === 'card_moved'" class="trigger-config">
                <select v-model="newAutomation.trigger_config.target_list_id">
                  <option value="">Select list...</option>
                  <option v-for="lwc in currentBoard?.lists" :key="lwc.list.id" :value="lwc.list.id">
                    {{ lwc.list.name }}
                  </option>
                </select>
              </div>
              <div v-if="newAutomation.trigger_type === 'label_added'" class="trigger-config">
                <select v-model="newAutomation.trigger_config.label_id">
                  <option value="">Select label...</option>
                  <option v-for="label in currentBoard?.labels" :key="label.id" :value="label.id">
                    {{ label.name }}
                  </option>
                </select>
              </div>
            </div>
            <div class="form-group">
              <label>Then...</label>
              <select v-model="newAutomation.action_type">
                <option value="move_to_list">Move card to list</option>
                <option value="add_label">Add label</option>
                <option value="remove_label">Remove label</option>
                <option value="archive_card">Archive card</option>
              </select>
              <div v-if="newAutomation.action_type === 'move_to_list'" class="action-config">
                <select v-model="newAutomation.action_config.target_list_id">
                  <option value="">Select list...</option>
                  <option v-for="lwc in currentBoard?.lists" :key="lwc.list.id" :value="lwc.list.id">
                    {{ lwc.list.name }}
                  </option>
                </select>
              </div>
              <div v-if="newAutomation.action_type === 'add_label' || newAutomation.action_type === 'remove_label'" class="action-config">
                <select v-model="newAutomation.action_config.label_id">
                  <option value="">Select label...</option>
                  <option v-for="label in currentBoard?.labels" :key="label.id" :value="label.id">
                    {{ label.name }}
                  </option>
                </select>
              </div>
            </div>
            <button class="btn btn-primary" @click="createAutomation" :disabled="!isAutomationValid">Create Automation</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Confirmation Modals -->
    <ConfirmModal
      :visible="deleteListModalVisible"
      title="Delete List"
      message="Delete this list and all its cards? This action cannot be undone."
      confirm-text="Delete"
      variant="danger"
      @confirm="handleDeleteList"
      @cancel="deleteListModalVisible = false"
    />

    <ConfirmModal
      :visible="deleteCardModalVisible"
      title="Delete Card"
      message="Delete this card? This action cannot be undone."
      confirm-text="Delete"
      variant="danger"
      @confirm="handleDeleteCard"
      @cancel="deleteCardModalVisible = false"
    />

    <ConfirmModal
      :visible="deleteBoardModalVisible"
      title="Delete Board"
      message="Delete this board and all its lists and cards? This action cannot be undone."
      confirm-text="Delete"
      variant="danger"
      @confirm="handleDeleteBoard"
      @cancel="deleteBoardModalVisible = false"
    />

    <ConfirmModal
      :visible="deleteLabelModal.visible"
      title="Delete Label"
      message="Delete this label? It will be removed from all cards."
      confirm-text="Delete"
      variant="danger"
      @confirm="confirmDeleteLabel"
      @cancel="deleteLabelModal.visible = false"
    />

    <!-- Prompt Modal for board folders -->
    <PromptModal
      :visible="promptModal.visible"
      :title="promptModal.title"
      :placeholder="promptModal.placeholder"
      confirm-text="Create"
      @submit="handlePromptSubmit"
      @cancel="promptModal.visible = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { boardsApi, listsApi, cardsApi, labelsApi, automationsApi } from '../api'
import { useExplorerStore, type ExplorerItem } from '../stores/explorer'
import { useNotesStore } from '../stores/notes'
import type { Board, BoardWithLists, Card, List, BoardLabel } from '../types'
import { WorkspaceSidebar, MobileSidebarToggle, MobileOverlay, ResizeHandle } from '../components/workspace'
import ExplorerTree from '../components/ExplorerTree.vue'
import Icon from '../components/Icon.vue'
import ConfirmModal from '../components/ConfirmModal.vue'
import PromptModal from '../components/PromptModal.vue'

const router = useRouter()
const explorerStore = useExplorerStore()
const notesStore = useNotesStore()

// State
const selectedBoardId = ref<string | null>(null)
const currentBoard = ref<BoardWithLists | null>(null)
const sidebarWidth = ref(240)
const isResizing = ref(false)
const archivedLists = ref<List[]>([])
const archivedCards = ref<Card[]>([])
const isMobileSidebarOpen = ref(false)

// Board menu
const showBoardMenu = ref(false)
const menuWrapper = ref<HTMLElement | null>(null)

// Context menu
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  board: null as Board | null
})

// Board modal
const boardModal = ref({
  visible: false,
  isEdit: false,
  id: '',
  name: '',
  description: '',
  color: '#3498db'
})

// List modal
const showAddListModal = ref(false)
const newListName = ref('')

// Card modals
const showAddCardModal = ref(false)
const showEditCardModal = ref(false)
const selectedListId = ref('')
const newCard = ref({ title: '', description: '' })
const editingCard = ref<Card | null>(null)

// Side panels
const showLabelsPanel = ref(false)
const showArchivePanel = ref(false)
const showAutomationsPanel = ref(false)
const showFiltersPanel = ref(false)

// Labels
const newLabelName = ref('')
const newLabelColor = ref('#e74c3c')
const labelColorPalette = ['#e74c3c', '#3498db', '#2ecc71', '#f39c12', '#9b59b6', '#1abc9c', '#e67e22', '#34495e', '#fd79a8', '#00cec9']

// Filters
const selectedLabelFilters = ref<string[]>([])
const dueDateFilter = ref('')

// Automations
const newAutomation = ref({
  name: '',
  trigger_type: 'card_moved',
  trigger_config: {} as Record<string, unknown>,
  action_type: 'move_to_list',
  action_config: {} as Record<string, unknown>
})

// Drag and drop
const draggingCard = ref<Card | null>(null)
const draggingFromListId = ref<string>('')
const dragOverCard = ref<string>('')
const dropZoneListId = ref<string>('')
const noteDropTargetCardId = ref<string>('')
const boardContentDragOver = ref(false)

// Delete modals
const deleteListModalVisible = ref(false)
const deleteListId = ref('')
const deleteCardModalVisible = ref(false)
const deleteBoardModalVisible = ref(false)
const deleteLabelModal = ref({ visible: false, labelId: '' })

// Prompt modal for board folders
const promptModal = ref({
  visible: false,
  title: 'New Board Folder',
  placeholder: 'Enter folder name',
  action: 'board-folder',
  parentId: null as string | null
})

// Colors
const colorPalette = ['#3498db', '#2ecc71', '#e74c3c', '#f39c12', '#9b59b6', '#1abc9c', '#e67e22', '#34495e']

// Computed
const hasActiveFilters = computed(() => {
  return selectedLabelFilters.value.length > 0 || dueDateFilter.value !== ''
})

const filteredLists = computed(() => {
  if (!currentBoard.value) return []
  return currentBoard.value.lists.filter(lwc => !lwc.list.archived)
})

const isAutomationValid = computed(() => {
  if (!newAutomation.value.name.trim()) return false
  if (!newAutomation.value.trigger_type) return false
  if (!newAutomation.value.action_type) return false
  
  // Check trigger config
  if (newAutomation.value.trigger_type === 'card_moved' && !newAutomation.value.trigger_config.target_list_id) return false
  if (newAutomation.value.trigger_type === 'label_added' && !newAutomation.value.trigger_config.label_id) return false
  
  // Check action config
  if (newAutomation.value.action_type === 'move_to_list' && !newAutomation.value.action_config.target_list_id) return false
  if ((newAutomation.value.action_type === 'add_label' || newAutomation.value.action_type === 'remove_label') && !newAutomation.value.action_config.label_id) return false
  
  return true
})

// Filter cards
const getFilteredCards = (cards: Card[]) => {
  return cards.filter(card => {
    if (card.archived) return false
    
    // Label filter
    if (selectedLabelFilters.value.length > 0) {
      const hasLabel = selectedLabelFilters.value.some(labelId => card.labels.includes(labelId))
      if (!hasLabel) return false
    }
    
    // Due date filter
    if (dueDateFilter.value) {
      const now = new Date()
      const today = new Date(now.getFullYear(), now.getMonth(), now.getDate())
      const weekEnd = new Date(today.getTime() + 7 * 24 * 60 * 60 * 1000)
      
      switch (dueDateFilter.value) {
        case 'overdue':
          if (!card.due_date || new Date(card.due_date) >= now) return false
          break
        case 'today':
          if (!card.due_date) return false
          const dueDate = new Date(card.due_date)
          if (dueDate < today || dueDate >= new Date(today.getTime() + 24 * 60 * 60 * 1000)) return false
          break
        case 'week':
          if (!card.due_date) return false
          const dueDateWeek = new Date(card.due_date)
          if (dueDateWeek < today || dueDateWeek > weekEnd) return false
          break
        case 'none':
          if (card.due_date) return false
          break
      }
    }
    
    return true
  })
}

// Refresh tree
async function refreshTree() {
  await explorerStore.fetchAll()
}

// Mobile sidebar toggle
function toggleMobileSidebar() {
  isMobileSidebarOpen.value = !isMobileSidebarOpen.value
}

// Fetch single board with lists
async function fetchBoard(boardId: string) {
  try {
    const response = await boardsApi.get(boardId)
    currentBoard.value = response.data
  } catch (error) {
    console.error('Failed to fetch board:', error)
    currentBoard.value = null
  }
}

// Select a board
async function selectBoard(boardId: string) {
  selectedBoardId.value = boardId
  explorerStore.selectItem(boardId, 'board')
  await fetchBoard(boardId)
  // Close mobile sidebar when a board is selected
  isMobileSidebarOpen.value = false
}

// Explorer tree handlers
function handleExplorerSelect(item: ExplorerItem) {
  explorerStore.selectItem(item.id, item.type as 'note' | 'folder' | 'board' | 'board-folder')
  if (item.type === 'board') {
    selectBoard(item.id)
  }
}

function handleOpenNote(noteId: string) {
  router.push('/notes')
  notesStore.openNote(noteId)
}

function handleCreateNote(folderId: string | null) {
  // Navigate to notes and create note there
  localStorage.setItem('createNoteInFolder', folderId || '')
  router.push('/notes')
}

function handleCreateFolder(parentId: string | null) {
  // Navigate to notes and create folder there
  localStorage.setItem('createFolderInParent', parentId || '')
  router.push('/notes')
}

function handleRename(item: ExplorerItem) {
  if (item.type === 'board') {
    // For boards, use our own edit modal
    const board = explorerStore.boards.find(b => b.id === item.id)
    if (board) {
      boardModal.value = {
        visible: true,
        isEdit: true,
        id: board.id,
        name: board.name,
        description: board.description || '',
        color: board.color || '#3498db'
      }
    }
  } else if (item.type === 'board-folder') {
    const newName = prompt('Enter new folder name:', item.name)
    if (newName && newName !== item.name) {
      explorerStore.renameBoardFolder(item.id, newName)
    }
  } else {
    // For notes/folders, navigate to notes
    router.push('/notes')
  }
}

function handleExplorerDelete(item: ExplorerItem) {
  if (item.type === 'board') {
    selectedBoardId.value = item.id
    deleteBoardModalVisible.value = true
  } else if (item.type === 'board-folder') {
    // TODO: Add board folder deletion confirmation
    explorerStore.deleteBoardFolder(item.id)
  } else {
    // For notes/folders, navigate to notes
    router.push('/notes')
  }
}

async function handleCreateBoardFolder(parentId: string | null) {
  promptModal.value = {
    visible: true,
    title: 'New Board Folder',
    placeholder: 'Enter folder name',
    action: 'board-folder',
    parentId: parentId
  }
}

async function handlePromptSubmit(name: string) {
  if (name && promptModal.value.parentId !== undefined) {
    await explorerStore.createBoardFolder(name, promptModal.value.parentId)
    promptModal.value.visible = false
  }
}

async function handleDrop(data: { draggedId: string; draggedType: 'note' | 'folder' | 'board' | 'board-folder'; targetId: string | null; targetType: 'folder' | 'note' | 'board' | 'board-folder' | null; position: number; dropPosition: 'before' | 'after' | 'inside' }) {
  try {
    if (data.draggedType === 'note') {
      await explorerStore.moveNote(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchNotes()
    } else if (data.draggedType === 'folder') {
      await explorerStore.moveFolder(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchNotes()
    } else if (data.draggedType === 'board') {
      await explorerStore.moveBoard(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchBoards()
    } else if (data.draggedType === 'board-folder') {
      await explorerStore.moveBoardFolder(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchBoards()
    }
  } catch (error) {
    console.error('Failed to move item:', error)
  }
}

// Board menu
function toggleBoardMenu() {
  showBoardMenu.value = !showBoardMenu.value
}

function closeBoardMenuOnClickOutside(event: MouseEvent) {
  if (menuWrapper.value && !menuWrapper.value.contains(event.target as Node)) {
    showBoardMenu.value = false
  }
}

function handleContextAction(action: string) {
  const board = contextMenu.value.board
  contextMenu.value.visible = false
  
  if (!board) return
  
  if (action === 'edit') {
    boardModal.value = {
      visible: true,
      isEdit: true,
      id: board.id,
      name: board.name,
      description: board.description || '',
      color: board.color || '#3498db'
    }
  } else if (action === 'delete') {
    selectedBoardId.value = board.id
    deleteBoardModalVisible.value = true
  }
}

function closeContextMenu() {
  contextMenu.value.visible = false
}

// Kanban card context menu
function openCardContextMenu(event: MouseEvent, card: Card, _listId: string) {
  const menu = document.createElement('div')
  menu.className = 'context-menu'
  menu.style.cssText = `
    position: fixed;
    left: ${event.clientX}px;
    top: ${event.clientY}px;
    background: var(--bg-secondary, #fff);
    border: 1px solid var(--border-primary, #ddd);
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    padding: 4px 0;
    z-index: 1000;
    min-width: 160px;
  `

  const menuItems = [
    { label: 'Edit', action: () => openCardModal(card) },
    { label: 'Archive', action: () => archiveCard() },
    { label: 'Delete', action: () => { deleteCardModalVisible.value = true; editingCard.value = card } }
  ]

  menuItems.forEach(item => {
    const btn = document.createElement('button')
    btn.className = 'context-menu-item'
    btn.style.cssText = `
      width: 100%;
      padding: 8px 16px;
      border: none;
      background: none;
      text-align: left;
      cursor: pointer;
      color: var(--text-primary, #000);
      font-size: 14px;
    `
    btn.textContent = item.label
    btn.addEventListener('click', () => {
      item.action()
      document.body.removeChild(menu)
    })
    btn.addEventListener('mouseenter', () => {
      btn.style.background = 'var(--bg-hover, #f0f0f0)'
    })
    btn.addEventListener('mouseleave', () => {
      btn.style.background = 'none'
    })
    menu.appendChild(btn)
  })

  document.body.appendChild(menu)
  
  const closeMenu = () => {
    if (document.body.contains(menu)) {
      document.body.removeChild(menu)
    }
  }
  
  setTimeout(() => {
    document.addEventListener('click', closeMenu, { once: true })
  }, 0)
}

// Kanban list context menu
function openListContextMenu(event: MouseEvent, list: List) {
  const menu = document.createElement('div')
  menu.className = 'context-menu'
  menu.style.cssText = `
    position: fixed;
    left: ${event.clientX}px;
    top: ${event.clientY}px;
    background: var(--bg-secondary, #fff);
    border: 1px solid var(--border-primary, #ddd);
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    padding: 4px 0;
    z-index: 1000;
    min-width: 160px;
  `

  const menuItems = [
    { label: 'Edit', action: () => renameListPrompt(list) },
    { label: 'Archive', action: () => archiveList(list.id) },
    { label: 'Delete', action: () => confirmDeleteList(list.id) }
  ]

  menuItems.forEach(item => {
    const btn = document.createElement('button')
    btn.className = 'context-menu-item'
    btn.style.cssText = `
      width: 100%;
      padding: 8px 16px;
      border: none;
      background: none;
      text-align: left;
      cursor: pointer;
      color: var(--text-primary, #000);
      font-size: 14px;
    `
    btn.textContent = item.label
    btn.addEventListener('click', () => {
      item.action()
      document.body.removeChild(menu)
    })
    btn.addEventListener('mouseenter', () => {
      btn.style.background = 'var(--bg-hover, #f0f0f0)'
    })
    btn.addEventListener('mouseleave', () => {
      btn.style.background = 'none'
    })
    menu.appendChild(btn)
  })

  document.body.appendChild(menu)
  
  const closeMenu = () => {
    if (document.body.contains(menu)) {
      document.body.removeChild(menu)
    }
  }
  
  setTimeout(() => {
    document.addEventListener('click', closeMenu, { once: true })
  }, 0)
}

// Board CRUD
function openCreateBoardModal() {
  boardModal.value = {
    visible: false,
    isEdit: false,
    id: '',
    name: '',
    description: '',
    color: '#3498db'
  }
  boardModal.value.visible = true
}

function openEditBoardModal() {
  if (!currentBoard.value) return
  showBoardMenu.value = false
  boardModal.value = {
    visible: true,
    isEdit: true,
    id: currentBoard.value.board.id,
    name: currentBoard.value.board.name,
    description: currentBoard.value.board.description || '',
    color: currentBoard.value.board.color || '#3498db'
  }
}

function closeBoardModal() {
  boardModal.value.visible = false
}

async function saveBoardModal() {
  try {
    if (boardModal.value.isEdit) {
      await explorerStore.updateBoard(boardModal.value.id, {
        name: boardModal.value.name,
        description: boardModal.value.description,
        color: boardModal.value.color
      })
    } else {
      const board = await explorerStore.createBoard(
        boardModal.value.name,
        boardModal.value.description || undefined,
        boardModal.value.color
      )
      // Select the new board
      selectedBoardId.value = board.id
    }
    closeBoardModal()
    if (selectedBoardId.value) {
      await fetchBoard(selectedBoardId.value)
    }
  } catch (error) {
    console.error('Failed to save board:', error)
  }
}

function confirmDeleteBoard() {
  showBoardMenu.value = false
  deleteBoardModalVisible.value = true
}

async function handleDeleteBoard() {
  deleteBoardModalVisible.value = false
  if (!selectedBoardId.value) return
  
  try {
    await explorerStore.deleteBoard(selectedBoardId.value)
    selectedBoardId.value = null
    currentBoard.value = null
  } catch (error) {
    console.error('Failed to delete board:', error)
  }
}

// List CRUD
function openAddListModal() {
  showBoardMenu.value = false
  newListName.value = ''
  showAddListModal.value = true
}

function renameListPrompt(list: List) {
  const newName = prompt('Enter new list name:', list.name)
  if (newName && newName.trim() && newName !== list.name) {
    renameList(list.id, newName.trim())
  }
}

async function renameList(listId: string, newName: string) {
  try {
    await listsApi.update(listId, { name: newName })
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    console.error('Failed to rename list:', error)
  }
}

async function addList() {
  if (!currentBoard.value || !newListName.value.trim()) return
  
  try {
    await listsApi.create(currentBoard.value.board.id, { name: newListName.value })
    showAddListModal.value = false
    newListName.value = ''
    await fetchBoard(currentBoard.value.board.id)
  } catch (error) {
    console.error('Failed to add list:', error)
  }
}

function confirmDeleteList(listId: string) {
  deleteListId.value = listId
  deleteListModalVisible.value = true
}

async function handleDeleteList() {
  deleteListModalVisible.value = false
  try {
    await listsApi.delete(deleteListId.value)
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    console.error('Failed to delete list:', error)
  }
}

async function archiveList(listId: string) {
  try {
    await listsApi.archive(listId)
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    console.error('Failed to archive list:', error)
  }
}

async function restoreList(listId: string) {
  try {
    await listsApi.restore(listId)
    if (currentBoard.value) {
      await Promise.all([fetchBoard(currentBoard.value.board.id), fetchArchivedItems()])
    }
  } catch (error) {
    console.error('Failed to restore list:', error)
  }
}

// Card CRUD
function openAddCardModal(listId: string) {
  selectedListId.value = listId
  newCard.value = { title: '', description: '' }
  showAddCardModal.value = true
}

async function addCard() {
  if (!newCard.value.title.trim()) return
  
  try {
    await cardsApi.create(selectedListId.value, newCard.value)
    showAddCardModal.value = false
    newCard.value = { title: '', description: '' }
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    console.error('Failed to add card:', error)
  }
}

function openCardModal(card: Card) {
  editingCard.value = { ...card }
  showEditCardModal.value = true
}

function closeEditCardModal() {
  showEditCardModal.value = false
  editingCard.value = null
}

async function updateCard() {
  if (!editingCard.value) return
  
  try {
    await cardsApi.update(editingCard.value.id, {
      title: editingCard.value.title,
      description: editingCard.value.description,
      due_date: editingCard.value.due_date,
      labels: editingCard.value.labels
    })
    closeEditCardModal()
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    console.error('Failed to update card:', error)
  }
}

function confirmDeleteCard() {
  deleteCardModalVisible.value = true
}

async function handleDeleteCard() {
  deleteCardModalVisible.value = false
  if (!editingCard.value) return
  
  try {
    await cardsApi.delete(editingCard.value.id)
    closeEditCardModal()
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    console.error('Failed to delete card:', error)
  }
}

async function archiveCard() {
  if (!editingCard.value) return
  try {
    await cardsApi.archive(editingCard.value.id)
    closeEditCardModal()
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    console.error('Failed to archive card:', error)
  }
}

async function restoreCard(cardId: string) {
  try {
    await cardsApi.restore(cardId)
    if (currentBoard.value) {
      await Promise.all([fetchBoard(currentBoard.value.board.id), fetchArchivedItems()])
    }
  } catch (error) {
    console.error('Failed to restore card:', error)
  }
}

function toggleCardLabel(labelId: string) {
  if (!editingCard.value) return
  const index = editingCard.value.labels.indexOf(labelId)
  if (index === -1) {
    editingCard.value.labels.push(labelId)
  } else {
    editingCard.value.labels.splice(index, 1)
  }
}

// Archive Panel
async function fetchArchivedItems() {
  if (!currentBoard.value) return
  try {
    const [listsRes, cardsRes] = await Promise.all([
      listsApi.getArchived(currentBoard.value.board.id),
      cardsApi.getArchived(currentBoard.value.board.id)
    ])
    archivedLists.value = listsRes.data
    archivedCards.value = cardsRes.data
  } catch (error) {
    console.error('Failed to fetch archived items:', error)
  }
}

async function openArchivePanel() {
  await fetchArchivedItems()
  showArchivePanel.value = true
}

// Label operations
async function createLabel() {
  if (!currentBoard.value || !newLabelName.value.trim()) return
  try {
    await labelsApi.create(currentBoard.value.board.id, {
      name: newLabelName.value.trim(),
      color: newLabelColor.value
    })
    newLabelName.value = ''
    await fetchBoard(currentBoard.value.board.id)
  } catch (error) {
    console.error('Failed to create label:', error)
  }
}

function editLabel(label: BoardLabel) {
  const newName = prompt('Edit label name:', label.name)
  if (newName && newName.trim() && currentBoard.value) {
    labelsApi.update(label.id, { name: newName.trim() }).then(() => fetchBoard(currentBoard.value!.board.id))
  }
}

function deleteLabelById(labelId: string) {
  deleteLabelModal.value = { visible: true, labelId }
}

async function confirmDeleteLabel() {
  const labelId = deleteLabelModal.value.labelId
  deleteLabelModal.value.visible = false
  try {
    await labelsApi.delete(labelId)
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    console.error('Failed to delete label:', error)
  }
}

function getLabelColor(labelId: string): string {
  const label = currentBoard.value?.labels.find((l: BoardLabel) => l.id === labelId)
  return label?.color || '#888'
}

function getLabelName(labelId: string): string {
  const label = currentBoard.value?.labels.find((l: BoardLabel) => l.id === labelId)
  return label?.name || ''
}

// Filter operations
function toggleLabelFilter(labelId: string) {
  const index = selectedLabelFilters.value.indexOf(labelId)
  if (index === -1) {
    selectedLabelFilters.value.push(labelId)
  } else {
    selectedLabelFilters.value.splice(index, 1)
  }
}

function clearFilters() {
  selectedLabelFilters.value = []
  dueDateFilter.value = ''
}

// Automation operations
async function createAutomation() {
  if (!currentBoard.value || !isAutomationValid.value) return
  try {
    await automationsApi.create(currentBoard.value.board.id, {
      name: newAutomation.value.name,
      trigger_type: newAutomation.value.trigger_type,
      trigger_config: newAutomation.value.trigger_config,
      action_type: newAutomation.value.action_type,
      action_config: newAutomation.value.action_config
    })
    newAutomation.value = {
      name: '',
      trigger_type: 'card_moved',
      trigger_config: {},
      action_type: 'move_to_list',
      action_config: {}
    }
    await fetchBoard(currentBoard.value.board.id)
  } catch (error) {
    console.error('Failed to create automation:', error)
  }
}

async function toggleAutomation(id: string) {
  try {
    await automationsApi.toggle(id)
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    console.error('Failed to toggle automation:', error)
  }
}

async function deleteAutomation(id: string) {
  try {
    await automationsApi.delete(id)
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    console.error('Failed to delete automation:', error)
  }
}

function getTriggerLabel(type: string): string {
  const labels: Record<string, string> = {
    card_moved: 'card is moved to a list',
    due_date_passed: 'due date passes',
    label_added: 'label is added',
    card_created: 'card is created'
  }
  return labels[type] || type
}

function getActionLabel(type: string): string {
  const labels: Record<string, string> = {
    move_to_list: 'move card to list',
    add_label: 'add label',
    remove_label: 'remove label',
    archive_card: 'archive card'
  }
  return labels[type] || type
}

// Drag and drop handlers
function onCardDragStart(event: DragEvent, card: Card, listId: string) {
  if (!event.dataTransfer) return
  draggingCard.value = card
  draggingFromListId.value = listId
  event.dataTransfer.effectAllowed = 'move'
  event.dataTransfer.setData('text/plain', card.id)
}

function onCardDragEnd() {
  draggingCard.value = null
  draggingFromListId.value = ''
  dragOverCard.value = ''
  dropZoneListId.value = ''
  noteDropTargetCardId.value = ''
}

function onCardDragOver(event: DragEvent, cardId: string, _index: number, _listId: string) {
  dragOverCard.value = cardId
  
  // Check if this is a note/board being dragged from explorer
  if (event.dataTransfer?.types.includes('application/x-explorer-item')) {
    noteDropTargetCardId.value = cardId
    event.dataTransfer.dropEffect = 'link'
  }
}

function onCardDragLeave() {
  dragOverCard.value = ''
  noteDropTargetCardId.value = ''
}

function onListDragOver(_event: DragEvent, listId: string) {
  dropZoneListId.value = listId
}

async function onCardDrop(event: DragEvent, targetListId: string) {
  event.preventDefault()
  if (!draggingCard.value || !currentBoard.value) return
  
  const targetList = currentBoard.value.lists.find(lwc => lwc.list.id === targetListId)
  if (!targetList) return
  
  // Get position at end of list
  const filteredCards = getFilteredCards(targetList.cards)
  const position = filteredCards.length
  
  try {
    await cardsApi.move({
      card_id: draggingCard.value.id,
      target_list_id: targetListId,
      position
    })
    await fetchBoard(currentBoard.value.board.id)
  } catch (error) {
    console.error('Failed to move card:', error)
  }
  
  onCardDragEnd()
}

async function onCardDropOnCard(event: DragEvent, targetCardId: string, targetIndex: number, targetListId: string) {
  event.preventDefault()
  
  // Check if a note/board is being dropped to create a link
  const explorerData = event.dataTransfer?.getData('application/x-explorer-item')
  if (explorerData) {
    try {
      const { id, type, name } = JSON.parse(explorerData)
      // Find the card and append a link to its description
      const targetList = currentBoard.value?.lists.find(lwc => lwc.list.id === targetListId)
      const card = targetList?.cards.find(c => c.id === targetCardId)
      if (card) {
        const linkType = type === 'note' ? 'notes' : 'boards'
        const linkHtml = `\n\n📎 [${name}](${linkType}://${id})`
        const newDescription = (card.description || '') + linkHtml
        await cardsApi.update(card.id, { description: newDescription })
        await fetchBoard(currentBoard.value!.board.id)
      }
    } catch (e) {
      console.error('Failed to create link:', e)
    }
    noteDropTargetCardId.value = ''
    return
  }
  
  if (!draggingCard.value || !currentBoard.value) return
  
  try {
    await cardsApi.move({
      card_id: draggingCard.value.id,
      target_list_id: targetListId,
      position: targetIndex
    })
    await fetchBoard(currentBoard.value.board.id)
  } catch (error) {
    console.error('Failed to move card:', error)
  }
  
  onCardDragEnd()
}

async function onCardDropAtEnd(event: DragEvent, targetListId: string) {
  event.preventDefault()
  if (!draggingCard.value || !currentBoard.value) return
  
  const targetList = currentBoard.value.lists.find(lwc => lwc.list.id === targetListId)
  if (!targetList) return
  
  const filteredCards = getFilteredCards(targetList.cards)
  const position = filteredCards.length
  
  try {
    await cardsApi.move({
      card_id: draggingCard.value.id,
      target_list_id: targetListId,
      position
    })
    await fetchBoard(currentBoard.value.board.id)
  } catch (error) {
    console.error('Failed to move card:', error)
  }
  
  onCardDragEnd()
}

// Board content drag handlers for opening notes/boards
function onBoardContentDragEnter(event: DragEvent) {
  const types = event.dataTransfer?.types || []
  const typesArray = Array.from(types)
  const isExplorer = typesArray.includes('application/x-explorer-item')
  if (!isExplorer) return
  
  event.preventDefault()
  boardContentDragOver.value = true
}

function onBoardContentDragOver(event: DragEvent) {
  const types = event.dataTransfer?.types || []
  const typesArray = Array.from(types)
  const isExplorer = typesArray.includes('application/x-explorer-item')
  if (!isExplorer) return
  
  event.preventDefault()
  if (event.dataTransfer) event.dataTransfer.dropEffect = 'copy'
  boardContentDragOver.value = true
}

function onBoardContentDragLeave(event: DragEvent) {
  // Only clear if we're leaving the board-content element entirely
  if ((event.target as HTMLElement).classList?.contains('board-content')) {
    boardContentDragOver.value = false
  }
}

function onBoardContentDrop(event: DragEvent) {
  event.preventDefault()
  boardContentDragOver.value = false
  
  // Handle tree item drops (notes/boards from explorer)
  const treeData = event.dataTransfer?.getData('application/x-explorer-item')
  
  if (treeData) {
    try {
      let parsed: any
      if (treeData.startsWith('{')) {
        parsed = JSON.parse(treeData)
      } else {
        // fallback when only id is set
        parsed = { id: treeData, type: 'note' }
      }
      const { id, type } = parsed
      
      if (type === 'note') {
        // Open the note in notes workspace
        notesStore.openNote(id)
      } else if (type === 'board') {
        // Switch to the board
        const boards = explorerStore.boards
        const targetBoard = findBoardById(boards, id)
        if (targetBoard) {
          currentBoard.value = targetBoard
          localStorage.setItem('selectedBoardId', id)
        }
      }
    } catch (e) {
      console.error('Failed to parse dropped item:', e)
    }
  }
}

// Helper to find board by id recursively
function findBoardById(boards: any[], id: string): any {
  for (const board of boards) {
    if (board.board.id === id) return board
    if (board.children && board.children.length > 0) {
      const found = findBoardById(board.children, id)
      if (found) return found
    }
  }
  return null
}

// Helper to check if a card has note links
function hasNoteLinks(description: string | undefined): boolean {
  if (!description) return false
  return description.includes('notes://') || description.includes('boards://')
}

// Get linked items from card description
interface LinkedItem {
  type: 'note' | 'board'
  name: string
  id: string
  href: string
  fullMatch: string
}

function getLinkedItems(description: string | undefined): LinkedItem[] {
  if (!description) return []
  
  const items: LinkedItem[] = []
  // Match markdown-style links: 📎 [name](notes://id) or [name](boards://id)
  const linkPattern = /📎?\s*\[([^\]]+)\]\((notes|boards):\/\/([^)]+)\)/g
  let match
  
  while ((match = linkPattern.exec(description)) !== null) {
    items.push({
      type: match[2] as 'note' | 'board',
      name: match[1],
      id: match[3],
      href: `${match[2]}://${match[3]}`,
      fullMatch: match[0]
    })
  }
  
  return items
}

function navigateToLinkedItem(item: LinkedItem) {
  closeEditCardModal()
  
  if (item.type === 'note') {
    localStorage.setItem('openNoteId', item.id)
    router.push('/notes')
  } else {
    selectBoard(item.id)
  }
}

function removeLinkedItem(item: LinkedItem) {
  if (!editingCard.value) return
  
  // Remove the link from description
  const newDescription = editingCard.value.description?.replace(item.fullMatch, '').trim() || ''
  editingCard.value.description = newDescription
}

// Helpers
function getPreview(text: string): string {
  if (!text) return ''
  // Strip out link markdown for preview
  const cleanText = text.replace(/📎 \[.*?\]\(.*?\)/g, '').trim()
  return cleanText.length > 80 ? cleanText.substring(0, 80) + '...' : cleanText
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })
}

function isOverdue(dateStr: string): boolean {
  return new Date(dateStr) < new Date()
}

// Resize
function startSidebarResize() {
  isResizing.value = true
  document.addEventListener('mousemove', onResize)
  document.addEventListener('mouseup', stopResize)
}

function onResize(event: MouseEvent) {
  if (isResizing.value) {
    sidebarWidth.value = Math.max(180, Math.min(400, event.clientX))
  }
}

function stopResize() {
  isResizing.value = false
  document.removeEventListener('mousemove', onResize)
  document.removeEventListener('mouseup', stopResize)
}

// Lifecycle
onMounted(async () => {
  await explorerStore.fetchAll()
  document.addEventListener('click', closeContextMenu)
  document.addEventListener('click', closeBoardMenuOnClickOutside)
  
  // Check if a board was selected from another view
  const storedBoardId = localStorage.getItem('selectedBoardId')
  if (storedBoardId) {
    localStorage.removeItem('selectedBoardId')
    await selectBoard(storedBoardId)
  }
})

onUnmounted(() => {
  document.removeEventListener('click', closeContextMenu)
  document.removeEventListener('click', closeBoardMenuOnClickOutside)
})
</script>

<style scoped>
.boards-workspace {
  display: flex;
  height: calc(100vh - 56px);
  background: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
}

/* Board Content */
.board-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.board-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1.25rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex: 1;
  min-width: 0;
}

.header-left h2 {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  white-space: nowrap;
}

.board-description {
  font-size: 13px;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.15s;
}

.btn-icon:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.menu-wrapper {
  position: relative;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 0.25rem;
  min-width: 160px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  z-index: 100;
  overflow: hidden;
}

.dropdown-menu button {
  display: flex;
  align-items: center;
  gap: 0.625rem;
  width: 100%;
  padding: 0.5rem 0.875rem;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  text-align: left;
}

.dropdown-menu button:hover {
  background: var(--bg-hover);
}

.dropdown-menu button.danger {
  color: var(--danger);
}

.dropdown-menu button.danger:hover {
  background: rgba(248, 81, 73, 0.1);
}

.menu-divider {
  height: 1px;
  background: var(--border-primary);
  margin: 0.25rem 0;
}

/* Kanban Board */
.kanban-container {
  flex: 1;
  overflow-x: auto;
  padding: 1rem;
}

.lists-wrapper {
  display: flex;
  gap: 1rem;
  height: 100%;
  align-items: flex-start;
}

.kanban-list {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 0.75rem;
  min-width: 280px;
  max-width: 280px;
  display: flex;
  flex-direction: column;
  max-height: calc(100vh - 180px);
}

.list-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0 0.25rem 0.75rem;
  border-bottom: 1px solid var(--border-primary);
  margin-bottom: 0.75rem;
}

.list-header h3 {
  flex: 1;
  margin: 0;
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--text-primary);
}

.card-count {
  background: var(--bg-tertiary);
  color: var(--text-muted);
  font-size: 11px;
  padding: 0.125rem 0.5rem;
  border-radius: 10px;
}

.list-menu-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  border-radius: 4px;
  opacity: 0;
  transition: all 0.15s;
}

.kanban-list:hover .list-menu-btn {
  opacity: 1;
}

.list-menu-btn:hover {
  background: var(--bg-hover);
  color: var(--danger);
}

.cards-container {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.kanban-card {
  position: relative;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  padding: 0.75rem;
  cursor: pointer;
  transition: all 0.15s;
}

.kanban-card:hover {
  border-color: var(--accent);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.card-edit-btn {
  position: absolute;
  top: 0.375rem;
  right: 0.375rem;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  color: var(--text-muted);
  cursor: pointer;
  opacity: 0;
  transition: all 0.15s;
  z-index: 10;
}

.kanban-card:hover .card-edit-btn {
  opacity: 1;
}

.card-edit-btn:hover {
  background: var(--accent);
  border-color: var(--accent);
  color: white;
}

.kanban-card h4 {
  margin: 0;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
}

.card-labels {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
  margin-bottom: 0.5rem;
}

.label {
  padding: 0.125rem 0.5rem;
  border-radius: 3px;
  font-size: 10px;
  color: white;
  font-weight: 500;
}

.label.removable {
  cursor: pointer;
}

.card-description {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 0.375rem;
  line-height: 1.4;
}

.card-footer {
  margin-top: 0.5rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.due-date {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.125rem 0.375rem;
  background: var(--bg-hover);
  border-radius: 3px;
  font-size: 11px;
  color: var(--text-secondary);
}

.due-date.overdue {
  background: rgba(248, 81, 73, 0.15);
  color: var(--danger);
}

.add-card-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.375rem;
  background: transparent;
  border: 1px dashed var(--border-primary);
  border-radius: 6px;
  padding: 0.625rem;
  margin-top: 0.5rem;
  color: var(--text-muted);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s;
}

.add-card-btn:hover {
  background: var(--bg-hover);
  border-color: var(--accent);
  color: var(--text-primary);
}

.add-list-placeholder {
  background: var(--bg-tertiary);
  border: 2px dashed var(--border-primary);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 2rem 1rem;
  cursor: pointer;
  color: var(--text-muted);
  transition: all 0.15s;
}

.add-list-placeholder:hover {
  background: var(--bg-hover);
  border-color: var(--accent);
  color: var(--text-primary);
}

/* Empty State */
.board-empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.empty-content {
  text-align: center;
  color: var(--text-muted);
}

.empty-content h3 {
  margin: 1rem 0 0.5rem;
  color: var(--text-primary);
}

.empty-content p {
  margin-bottom: 1.5rem;
  font-size: 14px;
}

.empty-content .btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
}

/* Context Menu */
.context-menu {
  position: fixed;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  padding: 0.375rem 0;
  min-width: 140px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
  z-index: 1000;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 0.625rem;
  padding: 0.5rem 0.875rem;
  cursor: pointer;
  font-size: 13px;
  color: var(--text-primary);
  transition: background 0.15s;
}

.context-menu-item:hover {
  background: var(--bg-hover);
}

.context-menu-item.danger {
  color: var(--danger);
}

.context-menu-item.danger:hover {
  background: rgba(241, 76, 76, 0.1);
}

.context-menu-divider {
  height: 1px;
  background: var(--border-primary);
  margin: 0.375rem 0;
}

/* Modals */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 12px;
  padding: 1.5rem;
  min-width: 400px;
  max-width: 500px;
  box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
}

.modal-large {
  max-width: 600px;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.25rem;
}

.modal-header h3 {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--text-primary);
}

.modal-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.15s;
}

.modal-close:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.375rem;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 0.625rem 0.875rem;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 14px;
  transition: border-color 0.15s;
}

.form-group input::placeholder,
.form-group textarea::placeholder {
  color: var(--text-muted);
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: var(--accent);
}

.color-picker {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.color-option {
  width: 28px;
  height: 28px;
  border-radius: 6px;
  border: 2px solid transparent;
  cursor: pointer;
  transition: transform 0.15s;
}

.color-option:hover {
  transform: scale(1.1);
}

.color-option.selected {
  border-color: var(--text-primary);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  margin-top: 1.5rem;
}

.labels-input {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.labels-input input {
  flex: 1;
}

.labels-list {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

/* Buttons */
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.375rem;
  padding: 0.625rem 1.25rem;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
}

.btn-primary {
  background: var(--accent);
  color: white;
}

.btn-primary:hover {
  background: var(--accent-hover);
}

.btn-secondary {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border-primary);
}

.btn-secondary:hover {
  background: var(--bg-hover);
}

.btn-danger {
  background: var(--danger);
  color: white;
}

.btn-danger:hover {
  filter: brightness(1.1);
}

.btn-warning {
  background: #f39c12;
  color: white;
}

.btn-warning:hover {
  background: #d68910;
}

.btn-primary:disabled {
  background: var(--text-muted);
  cursor: not-allowed;
}

.btn-icon.active {
  background: var(--accent);
  color: white;
}

/* Filters Bar */
.filters-bar {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  padding: 0.75rem 1.25rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
  flex-wrap: wrap;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.filter-group label {
  font-weight: 500;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.filter-labels {
  display: flex;
  gap: 0.25rem;
  flex-wrap: wrap;
}

.filter-label {
  padding: 0.2rem 0.5rem;
  border-radius: 3px;
  font-size: 0.75rem;
  cursor: pointer;
  border: 2px solid;
  background: transparent;
  color: var(--text-primary);
  transition: all 0.2s;
}

.filter-label.selected {
  color: white;
}

.filter-group select {
  padding: 0.3rem 0.5rem;
  border-radius: 4px;
  border: 1px solid var(--border-primary);
  background: var(--bg-tertiary);
  color: var(--text-primary);
  font-size: 0.85rem;
}

/* Drag and Drop */
.kanban-card.drag-over {
  border-top: 2px solid var(--accent);
}

.kanban-card.note-drop-target {
  background: color-mix(in srgb, var(--success) 15%, var(--bg-tertiary));
  border-color: var(--success);
  box-shadow: 0 0 0 2px var(--success);
}

.kanban-card[draggable="true"] {
  cursor: grab;
}

.kanban-card[draggable="true"]:active {
  cursor: grabbing;
}

/* Card links indicator */
.card-links {
  position: absolute;
  top: 0.375rem;
  right: 0.375rem;
}

.link-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  background: var(--bg-tertiary);
  border-radius: 4px;
  color: var(--accent);
}

.card-drop-zone {
  height: 4px;
  margin: 0.25rem 0;
  border-radius: 2px;
  transition: all 0.2s;
}

.card-drop-zone.active {
  height: 40px;
  background: rgba(52, 152, 219, 0.2);
  border: 2px dashed var(--accent);
}

/* Linked Items in Card Modal */
.linked-items-list {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.linked-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  background: var(--bg-tertiary);
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
}

.linked-item:hover {
  background: var(--bg-hover);
}

.linked-item-name {
  flex: 1;
  font-size: 0.875rem;
  color: var(--text-primary);
}

.linked-item-remove {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  background: transparent;
  border: none;
  border-radius: 4px;
  color: var(--text-muted);
  cursor: pointer;
  opacity: 0;
  transition: all 0.15s;
}

.linked-item:hover .linked-item-remove {
  opacity: 1;
}

.linked-item-remove:hover {
  background: color-mix(in srgb, var(--danger) 20%, transparent);
  color: var(--danger);
}

/* Side Panels */
.side-panel-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: flex-end;
  z-index: 1000;
}

.side-panel {
  background: var(--bg-secondary);
  width: 350px;
  height: 100%;
  display: flex;
  flex-direction: column;
  box-shadow: -2px 0 10px rgba(0, 0, 0, 0.2);
}

.side-panel-wide {
  width: 450px;
}

.side-panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid var(--border-primary);
}

.side-panel-header h3 {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.side-panel-content {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
}

/* Labels Management */
.labels-list-manage {
  margin-bottom: 1.5rem;
}

.label-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.label-preview {
  flex: 1;
  padding: 0.5rem;
  border-radius: 4px;
  color: white;
  font-weight: 500;
  font-size: 0.875rem;
}

.label-actions {
  display: flex;
  gap: 0.25rem;
  margin-left: 0.5rem;
}

.create-label-form {
  background: var(--bg-tertiary);
  padding: 1rem;
  border-radius: 8px;
}

.create-label-form h4 {
  margin: 0 0 0.75rem;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.create-label-form input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  margin-bottom: 0.75rem;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

/* Label Selector in Edit Modal */
.label-selector {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.label-option {
  padding: 0.3rem 0.75rem;
  border-radius: 4px;
  border: 2px solid;
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.2s;
  background: transparent;
  color: var(--text-primary);
}

.label-option.selected {
  color: white;
}

/* Archive Panel */
.archive-section {
  margin-bottom: 1.5rem;
}

.archive-section h4 {
  margin: 0 0 0.75rem;
  color: var(--text-muted);
  font-size: 0.85rem;
}

.archived-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem;
  background: var(--bg-tertiary);
  border-radius: 4px;
  margin-bottom: 0.5rem;
}

.archived-item span {
  font-size: 0.875rem;
  color: var(--text-primary);
}

.empty-state {
  color: var(--text-muted);
  font-size: 0.85rem;
  padding: 0.5rem;
}

/* Automations Panel */
.automations-list {
  margin-bottom: 1.5rem;
}

.automation-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 0.75rem;
  background: var(--bg-tertiary);
  border-radius: 6px;
  margin-bottom: 0.5rem;
}

.automation-item.disabled {
  opacity: 0.6;
}

.automation-info {
  flex: 1;
}

.automation-name {
  font-weight: 600;
  margin-bottom: 0.25rem;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.automation-description {
  font-size: 0.8rem;
  color: var(--text-muted);
}

.automation-description strong {
  color: var(--text-secondary);
}

.automation-actions {
  display: flex;
  gap: 0.25rem;
}

.create-automation-form {
  background: var(--bg-tertiary);
  padding: 1rem;
  border-radius: 8px;
}

.create-automation-form h4 {
  margin: 0 0 1rem;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.create-automation-form .form-group {
  margin-bottom: 1rem;
}

.create-automation-form label {
  display: block;
  margin-bottom: 0.25rem;
  font-weight: 500;
  font-size: 0.85rem;
  color: var(--text-primary);
}

.create-automation-form input,
.create-automation-form select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.trigger-config,
.action-config {
  margin-top: 0.5rem;
}

/* ========================================
   RESPONSIVE DESIGN - TABLET (max-width: 1024px)
   ======================================== */
@media (max-width: 1024px) {
  .board-content {
    flex: 1;
    min-height: 0;
  }

  .kanban-list {
    min-width: 260px;
    max-width: 260px;
  }

  .board-header {
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 0.625rem 1rem;
  }

  .header-left {
    flex: 1 1 100%;
    min-width: 0;
  }

  .header-left h2 {
    font-size: 1rem;
  }

  .board-description {
    display: none;
  }

  .header-actions {
    flex: 1 1 100%;
    justify-content: flex-end;
  }

  .side-panel {
    width: 100%;
    max-width: 400px;
  }

  .side-panel-wide {
    width: 100%;
    max-width: 450px;
  }

  .modal {
    min-width: 90%;
    max-width: 90%;
    margin: 1rem;
  }

  .modal-large {
    max-width: 95%;
  }
}

/* ========================================
   RESPONSIVE DESIGN - SMARTPHONE (max-width: 768px)
   ======================================== */
@media (max-width: 768px) {
  .boards-workspace {
    position: relative;
    height: calc(100vh - 56px);
    height: calc(100dvh - 56px);
  }

  .board-content {
    width: 100%;
    height: 100%;
  }

  .board-header {
    padding: 0.5rem 0.75rem;
    gap: 0.5rem;
  }

  .header-left h2 {
    font-size: 0.9rem;
    white-space: normal;
    line-height: 1.3;
  }

  .header-actions {
    gap: 0.25rem;
  }

  .btn-icon {
    width: 36px;
    height: 36px;
  }

  /* Filters bar mobile */
  .filters-bar {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 0.625rem 0.75rem;
  }

  .filter-group {
    width: 100%;
    flex-wrap: wrap;
  }

  .filter-labels {
    width: 100%;
  }

  /* Kanban mobile - horizontal scroll with full-width cards option */
  .kanban-container {
    padding: 0.75rem;
  }

  .lists-wrapper {
    gap: 0.75rem;
    padding-bottom: 0.5rem;
  }

  .kanban-list {
    min-width: 85vw;
    max-width: 85vw;
    max-height: calc(100vh - 220px);
    max-height: calc(100dvh - 220px);
  }

  .list-header {
    padding: 0 0.125rem 0.5rem;
  }

  .list-header h3 {
    font-size: 0.8rem;
  }

  .list-menu-btn {
    opacity: 1;
    width: 28px;
    height: 28px;
  }

  .cards-container {
    gap: 0.375rem;
  }

  .kanban-card {
    padding: 0.625rem;
  }

  .kanban-card h4 {
    font-size: 0.8rem;
  }

  .card-description {
    font-size: 11px;
  }

  .card-labels .label {
    font-size: 9px;
    padding: 0.1rem 0.375rem;
  }

  .add-card-btn {
    padding: 0.5rem;
    font-size: 12px;
  }

  .add-list-placeholder {
    min-width: 200px;
    max-width: 200px;
    padding: 1.5rem 1rem;
  }

  /* Side panels full-screen on mobile */
  .side-panel-overlay {
    align-items: flex-end;
  }

  .side-panel,
  .side-panel-wide {
    width: 100%;
    max-width: 100%;
    height: 85vh;
    height: 85dvh;
    border-radius: 16px 16px 0 0;
    animation: slideUpPanel 0.3s ease;
  }

  @keyframes slideUpPanel {
    from {
      transform: translateY(100%);
    }
    to {
      transform: translateY(0);
    }
  }

  .side-panel-header {
    padding: 1rem;
    border-bottom: 1px solid var(--border-primary);
  }

  .side-panel-content {
    padding: 0.75rem;
  }

  /* Labels management mobile */
  .label-item {
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .label-preview {
    flex: 1 1 100%;
  }

  .label-actions {
    margin-left: 0;
    width: 100%;
    justify-content: flex-end;
  }

  /* Archive panel mobile */
  .archived-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .archived-item button {
    align-self: flex-end;
  }

  /* Automations mobile */
  .automation-item {
    flex-direction: column;
    gap: 0.75rem;
  }

  .automation-actions {
    width: 100%;
    justify-content: flex-end;
  }

  .create-automation-form {
    padding: 0.75rem;
  }

  /* Modals mobile */
  .modal-overlay {
    align-items: flex-end;
    padding: 0;
  }

  .modal,
  .modal-large {
    min-width: 100%;
    max-width: 100%;
    max-height: 90vh;
    max-height: 90dvh;
    margin: 0;
    border-radius: 16px 16px 0 0;
    animation: slideUpModal 0.3s ease;
  }

  @keyframes slideUpModal {
    from {
      opacity: 0;
      transform: translateY(100%);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .modal-header {
    position: sticky;
    top: 0;
    background: var(--bg-secondary);
    z-index: 1;
    padding-bottom: 0.75rem;
    margin-bottom: 0.75rem;
  }

  .modal-actions {
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .modal-actions .btn {
    flex: 1 1 auto;
    min-width: 100px;
    text-align: center;
    justify-content: center;
  }

  /* Form groups mobile */
  .form-group input,
  .form-group textarea,
  .form-group select {
    font-size: 16px; /* Prevents zoom on iOS */
  }

  .color-picker {
    gap: 0.625rem;
  }

  .color-option {
    width: 32px;
    height: 32px;
  }

  /* Label selector mobile */
  .label-selector {
    gap: 0.375rem;
  }

  .label-option {
    padding: 0.375rem 0.625rem;
    font-size: 0.8rem;
  }

  /* Linked items mobile */
  .linked-item {
    padding: 0.625rem;
  }

  .linked-item-remove {
    opacity: 1;
    width: 24px;
    height: 24px;
  }

  /* Dropdown menu mobile */
  .dropdown-menu {
    position: fixed;
    left: 1rem !important;
    right: 1rem !important;
    bottom: 1rem;
    top: auto !important;
    margin-top: 0;
    max-width: none;
    border-radius: 12px;
  }

  .dropdown-menu button {
    padding: 0.75rem 1rem;
    font-size: 14px;
  }

  /* Context menu mobile */
  .context-menu {
    position: fixed;
    left: 1rem !important;
    right: 1rem !important;
    bottom: 1rem;
    top: auto !important;
    border-radius: 12px;
  }

  .context-menu-item {
    padding: 0.75rem 1rem;
  }

  /* Empty state mobile */
  .board-empty {
    padding: 1rem;
  }

  .empty-content {
    padding: 2rem 1rem;
  }

  .empty-content h3 {
    font-size: 1.1rem;
  }

  .empty-content p {
    font-size: 13px;
  }
}

/* ========================================
   RESPONSIVE DESIGN - SMALL PHONES (max-width: 480px)
   ======================================== */
@media (max-width: 480px) {
  .kanban-list {
    min-width: 90vw;
    max-width: 90vw;
  }

  .header-actions {
    gap: 0.125rem;
  }

  .btn-icon {
    width: 32px;
    height: 32px;
  }

  .board-header {
    padding: 0.375rem 0.5rem;
  }

  .header-left h2 {
    font-size: 0.85rem;
  }

  .kanban-container {
    padding: 0.5rem;
  }

  .kanban-list {
    padding: 0.5rem;
    max-height: calc(100vh - 200px);
    max-height: calc(100dvh - 200px);
  }

  .kanban-card {
    padding: 0.5rem;
  }

  .kanban-card h4 {
    font-size: 0.75rem;
  }

  .modal {
    padding: 1rem;
  }

  .modal-header h3 {
    font-size: 1rem;
  }

  .side-panel,
  .side-panel-wide {
    height: 90vh;
    height: 90dvh;
  }
}

/* ========================================
   TOUCH DEVICE OPTIMIZATIONS
   ======================================== */
@media (hover: none) and (pointer: coarse) {
  /* Make touch targets larger */
  .btn-icon {
    min-width: 44px;
    min-height: 44px;
  }

  .list-menu-btn {
    min-width: 44px;
    min-height: 44px;
    opacity: 1;
  }

  .kanban-card {
    padding: 0.75rem;
  }

  .tab-close,
  .linked-item-remove {
    opacity: 1;
    min-width: 44px;
    min-height: 44px;
  }

  .color-option {
    min-width: 44px;
    min-height: 44px;
  }

  .label-option,
  .filter-label {
    min-height: 44px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  /* Disable hover effects that don't work well on touch */
  .kanban-card:hover {
    border-color: var(--border-primary);
    box-shadow: none;
  }

  .kanban-card:active {
    border-color: var(--accent);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  }

  /* Improve scrolling */
  .cards-container,
  .lists-wrapper,
  .kanban-container {
    -webkit-overflow-scrolling: touch;
  }
}

/* ========================================
   LANDSCAPE PHONE OPTIMIZATIONS
   ======================================== */
@media (max-width: 768px) and (orientation: landscape) {
  .boards-workspace {
    height: calc(100vh - 48px);
    height: calc(100dvh - 48px);
  }

  .kanban-list {
    min-width: 45vw;
    max-width: 45vw;
    max-height: calc(100vh - 140px);
    max-height: calc(100dvh - 140px);
  }

  .modal,
  .modal-large {
    max-height: 85vh;
    max-height: 85dvh;
  }

  .side-panel,
  .side-panel-wide {
    height: 100%;
    width: 60%;
    max-width: 400px;
    border-radius: 0;
    animation: slideInRight 0.3s ease;
  }

  @keyframes slideInRight {
    from {
      transform: translateX(100%);
    }
    to {
      transform: translateX(0);
    }
  }
}
</style>
