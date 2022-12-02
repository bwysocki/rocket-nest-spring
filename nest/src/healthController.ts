import { Get, Controller, HttpCode } from '@nestjs/common';

@Controller('admin/health')
export class HealthController {
  @Get('alive')
  @HttpCode(200)
  alive(): string {
    return 'OK';
  }

}
