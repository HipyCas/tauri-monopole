<template>
  <navbar />
  <router-view v-slot="{ Component }">
    <transition :name="transitionName">
      <component :is="Component" />
    </transition>
  </router-view>
</template>

<script>
import Navbar from "@/components/Navbar.vue";

export default {
  components: { Navbar },
  data: () => ({
    transitionName: "slither",
  }),
  watch: {
    $route(to, from) {
      if (from.path === "/") {
        this.transitionName = "drain";
      } else {
        this.transitionName = "slither";
      }
    },
  },
};
</script>

<style lang="stylus">
html, body
  padding 0
  margin 0
  font-family sans-serif

.slither-enter-active, .slither-leave-active {
  transition: transform 1s;
}

.slither-enter, .slither-leave-to {
  transform: translateX(100%);
}

.slither-enter-to, .slither-leave {
  transform: translateX(0);
}

.drain-leave-active {
  transition: transform 1s;
}

.drain-leave-to {
  transform: translateX(-100%);
}

.drain-leave {
  transform: translateX(0);
}
</style>
