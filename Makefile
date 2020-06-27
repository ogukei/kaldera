
# compiles all the GLSL files in ./data

ifndef GLSLC
GLSLC=glslc
endif

SOURCE_DIR=data/shaders
BUILD_DIR=data/shaders

SOURCES_VERT=$(shell find $(SOURCE_DIR) -name '*.vert')
SOURCES_FRAG=$(shell find $(SOURCE_DIR) -name '*.frag')
SOURCES_RGEN=$(shell find $(SOURCE_DIR) -name '*.rgen')
SOURCES_RMISS=$(shell find $(SOURCE_DIR) -name '*.rmiss')
SOURCES_RCHIT=$(shell find $(SOURCE_DIR) -name '*.rchit')
SOURCES=$(SOURCES_VERT) $(SOURCES_FRAG) $(SOURCES_RGEN) $(SOURCES_RMISS) $(SOURCES_RCHIT)

OBJECTS_0=$(patsubst $(SOURCE_DIR)/%.vert, $(BUILD_DIR)/%.vert.spv, $(SOURCES))
OBJECTS_1=$(patsubst $(SOURCE_DIR)/%.frag, $(BUILD_DIR)/%.frag.spv, $(OBJECTS_0))
OBJECTS_2=$(patsubst $(SOURCE_DIR)/%.rgen, $(BUILD_DIR)/%.rgen.spv, $(OBJECTS_1))
OBJECTS_3=$(patsubst $(SOURCE_DIR)/%.rmiss, $(BUILD_DIR)/%.rmiss.spv, $(OBJECTS_2))
OBJECTS_4=$(patsubst $(SOURCE_DIR)/%.rchit, $(BUILD_DIR)/%.rchit.spv, $(OBJECTS_3))
OBJECTS=$(OBJECTS_4)

all: $(OBJECTS)
clean:
	rm -f $(BUILD_DIR)/*.spv

$(BUILD_DIR)/%.vert.spv: $(SOURCE_DIR)/%.vert
	$(GLSLC) --target-env vulkan1.2 \
	-c $< \
	-o $@

$(BUILD_DIR)/%.frag.spv: $(SOURCE_DIR)/%.frag
	$(GLSLC) --target-env vulkan1.2 \
	-c $< \
	-o $@

$(BUILD_DIR)/%.rgen.spv: $(SOURCE_DIR)/%.rgen
	$(GLSLC) --target-env vulkan1.2 \
	-c $< \
	-o $@

$(BUILD_DIR)/%.rmiss.spv: $(SOURCE_DIR)/%.rmiss
	$(GLSLC) --target-env vulkan1.2 \
	-c $< \
	-o $@

$(BUILD_DIR)/%.rchit.spv: $(SOURCE_DIR)/%.rchit
	$(GLSLC) --target-env vulkan1.2 \
	-c $< \
	-o $@
