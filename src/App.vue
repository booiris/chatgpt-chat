<script>
export let debug_msg_data = []
</script>

<script setup>
import { register } from 'vue-advanced-chat'
import { reactive, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri';
import { emit, listen } from '@tauri-apps/api/event'

const debug_msg = ref(debug_msg_data)

register()
const data = reactive({
  currentUserId: '1234',
  rooms: [
    {
      roomId: "1",
      roomName: "First Room",
      avatar: "https://66.media.tumblr.com/avatar_c6a8eae4303e_512.pnj",
      users: [
        { _id: '123', username: 'you' },
        { _id: '1234', username: 'me' }
      ]
    }
  ],
  messages: [],
  messagesLoaded: true,
  loadingRooms: false,
  roomsLoaded: true,
  roomId: null,
  showFiles: false,
  showAudio: false,
  showReactionEmojis: false,
  roomActions: [
    // { name: 'inviteUser', title: 'Invite User' },
    // { name: 'removeUser', title: 'Remove User' },
    // { name: 'deleteRoom', title: 'Delete Room' }
  ],
  styles: {
    general: {
      color: '#bbbbbb',
      colorSpinner: '#333',
      borderStyle: '1px solid #e1e4e8'
    },

    footer: {
      background: '#f8f9fa',
      backgroundReply: 'rgba(0, 0, 0, 0.08)'
    },

    icons: {
      search: '#9ca6af'
    }
  }
})

function fetchMessages({ room, options = {} }) {
  data.messagesLoaded = false

  setTimeout(() => {
    if (options.reset) {
      invoke("query", { req: { text: "" } }).then(
        (message) => {
          console.log(message);
          let content = message["text"]
          let date = new Date(message["time"] * 1000)
          data.messages = [
            ...data.messages,
            {
              _id: data.messages.length,
              content: content,
              senderId: '4321',
              timestamp: date.toString().substring(16, 21),
              date: date.toDateString(),
              disableActions: true,
              disableReactions: true,
            }
          ]
        }
      ).catch((error) => { console.error(error), debug_msg.value.push(error) });
      data.messagesLoaded = true
    }
    data.messagesLoaded = true
  })
}

function sendMessage(message) {
  data.messages = [
    ...data.messages,
    {
      _id: data.messages.length,
      content: message.content,
      senderId: data.currentUserId,
      timestamp: new Date().toString().substring(16, 21),
      date: new Date().toDateString(),
      disableActions: true,
      disableReactions: true,
    }
  ]
  invoke("query", { req: { text: message.content } }).then(
    (message) => {
      console.log(message);
      let content = message["text"]
      let date = new Date(message["time"] * 1000)
      data.messages = [
        ...data.messages,
        {
          _id: data.messages.length,
          content: content,
          senderId: '4321',
          timestamp: date.toString().substring(16, 21),
          date: date.toDateString(),
          disableActions: true,
          disableReactions: true,
        }
      ]
    }
  ).catch((error) => { console.error(error), debug_msg.value.push(error) });
}

</script>

<template>
  <vue-advanced-chat height="calc(100vh - 20px)" :current-user-id=data.currentUserId .rooms="data.rooms"
    .messages="data.messages" .loading-rooms="data.loadingRooms" .rooms-loaded="data.roomsLoaded"
    .messages-loaded="data.messagesLoaded" .rood-id="data.roomId" .show-files="data.showFiles"
    .show-audio="data.showAudio" .show-reaction-emojis="data.showReactionEmojis" .styles="data.styles"
    @send-message="sendMessage($event.detail[0])" @fetch-messages="fetchMessages($event.detail[0])" />
  <li v-for="msg in debug_msg">
    {{ msg }}
  </li>
</template>