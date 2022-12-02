import { Injectable } from '@nestjs/common';
import { readFile } from 'fs/promises';
import { SampleDto } from '../dto/sample.dto';

@Injectable()
export class FSService {

  private static JSON_NAME = 'sample.json';

  async read(): Promise<SampleDto> {
    const content = await readFile(FSService.JSON_NAME, 'utf8');
    const messages = JSON.parse(content);
    return messages;
  }

}
