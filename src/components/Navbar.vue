<template>
  <nav>
    <router-link class="link" to="/">Viewer</router-link>
    <router-link class="link" to="/settings">Settings</router-link>
  </nav>
</template>

<script>
export default {};
</script>

<style lang="stylus" scoped>
$black = #111111
$silver = #DDDDDD

nav
  position absolute
  top 0
  width 100vw
  display flex
  flex-direction row
  justify-content center
  z-index 80
  gap 1em
  color $black

.link
  background-color white
  padding 0.5em
  font-size 12px
  border-bottom-left-radius 0.5em
  border-bottom-right-radius 0.5em
  // color $black
  text-decoration none
  color inherit

.link:hover
  color white
  background-color $black

.link:visited
  // color $black
  text-decoration none
</style>
