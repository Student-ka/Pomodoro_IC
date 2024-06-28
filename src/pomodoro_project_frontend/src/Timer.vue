<template>
    <div>
      <p>Time: {{ minutes }} minutes</p>
    </div>
  </template>
  
  <script>
  import axios from 'axios';
  
  export default {
    data() {
      return {
        minutes: 0
      }
    },
    created() {
      axios.get('/api/v2/canister/{canister_id}/query', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        data: JSON.stringify({
          idl: 'get_timer'
        }),
      })
        .then(response => {
          this.minutes = response.data.result.minutes;
        })
        .catch(error => {
          console.error("There was an error!", error);
        });
    }
  }
  </script>
  