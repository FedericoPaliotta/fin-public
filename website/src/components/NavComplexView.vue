<template>
  <nav class="navbar" role="navigation" aria-label="main navigation">
    <div class="navbar-brand">
      <a class="navbar-item" href="/">
        <img src="https://via.placeholder.com/150" width="112" height="28" />
      </a>

      <a
        role="button"
        class="navbar-burger burger"
        aria-label="menu"
        aria-expanded="false"
        data-target="navbarBasicExample"
      >
        <span aria-hidden="true"></span>
        <span aria-hidden="true"></span>
        <span aria-hidden="true"></span>
      </a>
    </div>

    <div id="navbarBasicExample" class="navbar-menu">
      <div class="navbar-end">
        <a class="navbar-item" href="#" v-if="!isUserAuth" v-on:click="logout"
          >logout</a
        >
        <router-link v-if="isUserAuth" to="/login">
          <a class="navbar-item">login</a>
        </router-link>
        <!-- <router-link v-if="isUserAuth" to="/signup" -->
        <!--   ><a class="navbar-item">signup</a></router-link -->
        <!-- > -->
        <router-link to="/home"> <a class="navbar-item">home</a> </router-link>
        <!-- <router-link to="/portfolio"> <a>portfolio</a> </router-link> -->
      </div>
    </div>
  </nav>
</template>

<script lang="ts">
import Vue from "vue";
import axios from "axios";
import router from "../index.js";

const ax = axios.create({
  baseURL: "http://localhost:8000/users",
  timeout: 10000,
  withCredentials: true
  //headers: { "Access-Control-Max-Age": "1" },
});
ax.interceptors.response.use(
  function(response) {
    return response;
  },
  function(error) {
    if (401 === error.response.status) {
      router.push({ name: "login" });
      return Promise.reject(error);
    }
  }
);

export default Vue.extend({
  props: {
    isUserAuth: {
      type: Boolean,
      default: false
    }
  },
  mounted() {
    this.toggleNav();
  },
  methods: {
    toggleNav: function() {
      // Get all "navbar-burger" elements
      const $navbarBurgers = Array.prototype.slice.call(
        document.querySelectorAll(".navbar-burger"),
        0
      );

      // Check if there are any navbar burgers
      if ($navbarBurgers.length > 0) {
        // Add a click event on each of them
        $navbarBurgers.forEach(el => {
          el.addEventListener("click", () => {
            // Get the target from the "data-target" attribute
            const target = el.dataset.target;
            const $target = document.getElementById(target);

            // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
            el.classList.toggle("is-active");
            $target.classList.toggle("is-active");
          });
        });
      }
    },
    logout: function(event) {
      event.preventDefault();
      ax.post("/logout").catch(error => {});
      router.push({ name: "login" });
    }
  }
});
</script>

<style lang="scss" scoped></style>
