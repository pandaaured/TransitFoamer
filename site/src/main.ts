import './styles/base.css'
import './styles/nav.css'
import './styles/home.css'
import './styles/route-list.css'
import './styles/schedule.css'
import './styles/sequence.css'
import './styles/header.css'
import { mount } from 'svelte'
import App from './App.svelte'

mount(App, { target: document.getElementById('app')! })
