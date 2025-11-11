CXX=g++
CXXFLAGS=-I./memflow_lib/memflow-ffi/ -L./memflow_lib/target/release -Wno-multichar
LIBS=-lm -ldl -lpthread -lrt

OUTDIR=./build
OBJDIR=$(OUTDIR)/obj

$(shell mkdir -p $(OBJDIR))

%.o: %.cpp
	$(CXX) -c -o $(OBJDIR)/$@ $< $(CXXFLAGS)

apex_dma: apex_dma.o Game.o Math.o memory.o
	$(CXX) -o $(OUTDIR)/$@ $(OBJDIR)/apex_dma.o $(OBJDIR)/Game.o $(OBJDIR)/Math.o $(OBJDIR)/memory.o -L./memflow_lib/target/release -lmemflow_ffi $(LIBS) -Wl,-rpath,'$$ORIGIN/../memflow_lib/target/release'

.PHONY: all clean
all: apex_dma

.DEFAULT_GOAL := all

clean:
	rm -rf $(OUTDIR)
