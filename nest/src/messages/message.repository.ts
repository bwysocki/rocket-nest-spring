import { Injectable } from '@nestjs/common';
import { readFile, writeFile } from 'fs/promises';

@Injectable()
export class MessageRepository {

  private static JSON_NAME = 'messages.json';

  async findOne(id: number) {
    const messages = await this.getMessages()
    return messages[id];
  }

  async findAll() {
    return this.getMessages();
  }

  async create(message: string) {
    const messages = await this.getMessages();
    const id = Math.floor(Math.random() * 999);
    messages[id] = {
      "content": message,
      id
    }

    await writeFile(MessageRepository.JSON_NAME, JSON.stringify(messages));
  }

  private async getMessages() {
    const content = await readFile(MessageRepository.JSON_NAME, 'utf8');
    const messages = JSON.parse(content);
    return messages;
  }
}
