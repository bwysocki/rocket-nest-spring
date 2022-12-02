import { Injectable } from '@nestjs/common';
import { MessageRepository } from './message.repository';

@Injectable()
export class MessageService {

  constructor(public messageRepository: MessageRepository) {}

  async findOne(id: number) {
    return this.messageRepository.findOne(id);
  }

  async findAll() {
    return this.messageRepository.findAll();
  }

  async create(message: string) {
    this.messageRepository.create(message);
  }

}
