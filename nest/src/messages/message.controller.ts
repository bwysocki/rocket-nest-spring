import { Controller, Get, Post, Body, Param, NotFoundException  } from '@nestjs/common';
import { CreateMessageDto } from './dto/create-message.dto';
import { MessageService } from './message.service';

@Controller('message')
export class MessageController {


  constructor(public messageService: MessageService) {}

  @Get()
  listMessages() {
    return this.messageService.findAll();
  }

  @Get('/:id')
  async getMessage(@Param('id') id: number) {
    const message = await this.messageService.findOne(id);
    if (!message) {
      throw new NotFoundException(`Message with id = ${id} not found`);
    }
    return message;
  }

  @Post()
  createMessage(@Body() body: CreateMessageDto) {
    return this.messageService.create(body.content);
  }

}
