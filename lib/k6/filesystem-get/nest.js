import http from 'k6/http';
import { sleep } from 'k6';

export default function () {
  http.get('http://localhost:3001/message');
}

//k6 run --vus 10 --duration 30s nest.js
