import { Module } from '@nestjs/common';
import { MessageController } from './messages/message.controller';
import { HealthController } from './healthController';
import { FSController } from './file-system/controller/fs.controller';
import { FSService } from './file-system/service/fs.service';
import { MessageService } from './messages/message.service';
import { MessageRepository } from './messages/message.repository';
import {
  StatusMonitorModule,
  StatusMonitorConfiguration,
} from 'nestjs-status-monitor';

const SOCKET_IO_PORT = +process.env.EXTERNAL_PORT || +process.env.PORT || 3001;
const APP_PORT = +process.env.PORT || 3001;


const statusMonitorConfig: StatusMonitorConfiguration = {
  title: 'NestJS Monitoring Page',
  port: SOCKET_IO_PORT,
  socketPath: '/socket.io',
  path: '/status',
  ignoreStartsWith: '/admin',
  healthChecks: [
    {
      protocol: 'http',
      host: 'localhost',
      path: '/admin/health/alive',
      port: APP_PORT,
    },
  ],
  spans: [
    {
      interval: 1, // Every second
      retention: 60, // Keep 60 datapoints in memory
    },
    {
      interval: 5, // Every 5 seconds
      retention: 60,
    },
    {
      interval: 15, // Every 15 seconds
      retention: 60,
    },
  ],
  chartVisibility: {
    cpu: true,
    mem: true,
    load: true,
    responseTime: true,
    rps: true,
    statusCodes: true,
  },
};


@Module({
  imports: [StatusMonitorModule.forRoot()],
  controllers: [HealthController, MessageController, FSController],
  providers: [MessageService, MessageRepository, FSService]
})
export class NestjsModule {}
