import { Controller, Get, Query  } from '@nestjs/common';
import { FSService } from '../service/fs.service';
import { SampleDto } from '../dto/sample.dto';
import { ReadParamDto } from '../dto/read.dto';

@Controller('fs')
export class FSController {

  constructor(public service: FSService) { }

  @Get()
  async read(@Query() params: ReadParamDto): Promise<SampleDto> {
    const sample: SampleDto = await this.service.read();
    sample.extra = params.extra;
    return sample;
  }

}
