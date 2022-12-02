package pl.stalostech.rocketnestspring.controller;

import com.fasterxml.jackson.databind.ObjectMapper;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.util.ResourceUtils;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.server.ResponseStatusException;
import pl.stalostech.rocketnestspring.dto.Sample;

import static org.springframework.http.HttpStatus.NOT_FOUND;

@RestController
@Validated
public class FileSystemRestController {

    public static Logger logger = LoggerFactory.getLogger(FileSystemRestController.class);

    @GetMapping("/spring-filesystem-read")
    Sample read(@RequestParam(value = "extra") @NotBlank @Size(max = 100) String extra) {
        try {
            ObjectMapper mapper = new ObjectMapper();
            Sample json = mapper.readValue(ResourceUtils.getFile("classpath:sample.json"), Sample.class);
            json.setExtra(extra);
            return json;
        } catch (Exception e) {
            logger.error("Can not read response", e);
            throw new ResponseStatusException(NOT_FOUND, "Unable to find resource");
        }
    }
}
