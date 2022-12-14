import http from 'k6/http';
import { sleep } from 'k6';

export default function () {
  http.get('http://localhost:8080/spring-filesystem-read?extra=something');
}

//k6 run --vus 10 --duration 30s spring.js
//http://nix-on.blogspot.com/2015/01/java-jstat-how-to-visualize-garbage.html
