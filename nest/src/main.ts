import { NestFactory } from '@nestjs/core';
import { ValidationPipe } from '@nestjs/common';
import { NestjsModule } from './nestjs.module';

async function bootstrap() {
  const app = await NestFactory.create(NestjsModule);
  app.useGlobalPipes(
    new ValidationPipe()
  );
  await app.listen(3001);
}
bootstrap();
