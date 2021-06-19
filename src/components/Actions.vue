<template>
  <div id="container">
    <div id="bottom">
      <button id="btn-run" @click="$emit('run')">Run</button>
      <button id="btn-animate" @click="$emit('animate')">Animate</button>
      <button id="btn-stop-animate" @click="$emit('stop-animate')">
        Stop Animation
      </button>
    </div>
  </div>
</template>

<script>
export default {
  emits: ["run", "animate", "stop-animate"],
};
</script>

<style lang="stylus" scoped>
$green = #2ECC40
$lime = #01FF70
$red = #FF4136
$orange = #FF851B
$silver = #DDDDDD

#container
  position absolute
  height 100vh
  width 100vw
  z-index 10

#bottom
  position absolute
  bottom 0
  width 100vw
  display flex
  justify-content center
  gap 0.5em
  padding 0.5em 0

button
  border-radius 0.2em
  border-width 0
  text-transform uppercase
  padding 0.5em

#btn-run
  background-color $green

#btn-run:hover
  background-color $lime

#btn-animate:hover
  background-color $silver

#btn-stop-animate
  background-color $red

#btn-stop-animate:hover
  background-color $orange

#btn-stop-animate [disable]
  opacity 0.5
// TODO Disable when animation stopped
</style>
