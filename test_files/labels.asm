    SE V0, V1
    SNE V0, V1
    SHL VF
label0:
    LD B, V9
    LD VA, DT
    LD DT, VA
    LD K, V0
    LD F, V0
    JP label0
