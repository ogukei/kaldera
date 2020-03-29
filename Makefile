
# compiles all the GLSL files in ./data

SOURCE_DIR=data
BUILD_DIR=data

SOURCES_VERT=$(shell find $(SOURCE_DIR) -name '*.vert')
SOURCES_FRAG=$(shell find $(SOURCE_DIR) -name '*.frag')
SOURCES=$(SOURCES_VERT) $(SOURCES_FRAG)

OBJECTS_0=$(patsubst $(SOURCE_DIR)/%.vert, $(BUILD_DIR)/%.vert.spv, $(SOURCES))
OBJECTS_1=$(patsubst $(SOURCE_DIR)/%.frag, $(BUILD_DIR)/%.frag.spv, $(OBJECTS_0))
OBJECTS=$(OBJECTS_1)

all: $(OBJECTS)
clean:
	rm -f $(BUILD_DIR)/*.spv

$(BUILD_DIR)/%.vert.spv: $(SOURCE_DIR)/%.vert
	glslc -O \
	-c $< \
	--target-env=vulkan1.1 \
	--target-spv=spv1.3 \
	-o $@

$(BUILD_DIR)/%.frag.spv: $(SOURCE_DIR)/%.frag
	glslc -O \
	-c $< \
	--target-env=vulkan1.1 \
	--target-spv=spv1.3 \
	-o $@
