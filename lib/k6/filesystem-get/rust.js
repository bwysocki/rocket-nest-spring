import http from 'k6/http';
import { sleep } from 'k6';

export default function () {
  http.get('http://localhost:8000/api/rust-filesystem-read?extra=asd');
}

//k6 run --vus 10 --duration 30s rust.js
//http://nix-on.blogspot.com/2015/01/java-jstat-how-to-visualize-garbage.html
