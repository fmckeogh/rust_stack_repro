#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use u__id::*;
use Zeros::*;
use PhysicalCountInt::*;
use integer_subrange::*;
use common::*;
pub fn ImpDefBits<T: Tracer>(
    state: &mut State,
    tracer: &T,
    x: &'static str,
    N: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        gs_331400: bool,
        gs_331449: bool,
        return_value: Bits,
        gs_331441: bool,
        ga_370920: Bits,
        gs_331479: bool,
        gs_331501: bool,
        gs_331493: bool,
        Nshadow_8009: i128,
        gs_331509: bool,
        gs_331463: bool,
        gs_331471: bool,
        gs_331391: bool,
        x: &'static str,
        N: i128,
    }
    let fn_state = FunctionState {
        x,
        N,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var N:i
        let s_0_0: i128 = fn_state.N;
        // D s_0_1: write-var Nshadow#8009 <= s_0_0
        fn_state.Nshadow_8009 = s_0_0;
        // D s_0_2: read-var x:str
        let s_0_2: &'static str = fn_state.x;
        // C s_0_3: const #"Coresight timestamp" : str
        let s_0_3: &'static str = "Coresight timestamp";
        // D s_0_4: cmp-eq s_0_2 s_0_3
        let s_0_4: bool = ((s_0_2) == (s_0_3));
        // N s_0_5: branch s_0_4 b89 b1
        if s_0_4 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_1_0: read-var x:str
        let s_1_0: &'static str = fn_state.x;
        // C s_1_1: const #"Synchronous Error" : str
        let s_1_1: &'static str = "Synchronous Error";
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b85 b2
        if s_1_2 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var x:str
        let s_2_0: &'static str = fn_state.x;
        // C s_2_1: const #"Asynchronous Error" : str
        let s_2_1: &'static str = "Asynchronous Error";
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b81 b3
        if s_2_2 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_3_0: read-var x:str
        let s_3_0: &'static str = fn_state.x;
        // C s_3_1: const #"Virtual Asynchronous Abort ExT bit" : str
        let s_3_1: &'static str = "Virtual Asynchronous Abort ExT bit";
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b80 b4
        if s_3_2 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_4_0: read-var x:str
        let s_4_0: &'static str = fn_state.x;
        // C s_4_1: const #"FPEXC.EN value when TGE==1 and RW==0" : str
        let s_4_1: &'static str = "FPEXC.EN value when TGE==1 and RW==0";
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // N s_4_3: branch s_4_2 b79 b5
        if s_4_2 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_5_0: read-var x:str
        let s_5_0: &'static str = fn_state.x;
        // C s_5_1: const #"reset vector address" : str
        let s_5_1: &'static str = "reset vector address";
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // N s_5_3: branch s_5_2 b75 b6
        if s_5_2 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_6_0: read-var x:str
        let s_6_0: &'static str = fn_state.x;
        // C s_6_1: const #"TG0 encoded granule size" : str
        let s_6_1: &'static str = "TG0 encoded granule size";
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // N s_6_3: branch s_6_2 b71 b7
        if s_6_2 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_7_0: read-var x:str
        let s_7_0: &'static str = fn_state.x;
        // C s_7_1: const #"TG1 encoded granule size" : str
        let s_7_1: &'static str = "TG1 encoded granule size";
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b67 b8
        if s_7_2 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var x:str
        let s_8_0: &'static str = fn_state.x;
        // C s_8_1: const #"Reserved short descriptor AP encoding" : str
        let s_8_1: &'static str = "Reserved short descriptor AP encoding";
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b66 b9
        if s_8_2 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_9_0: read-var x:str
        let s_9_0: &'static str = fn_state.x;
        // C s_9_1: const #"Non-Faulting PAR" : str
        let s_9_1: &'static str = "Non-Faulting PAR";
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // N s_9_3: branch s_9_2 b62 b10
        if s_9_2 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_10_0: read-var x:str
        let s_10_0: &'static str = fn_state.x;
        // C s_10_1: const #"Faulting PAR" : str
        let s_10_1: &'static str = "Faulting PAR";
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // N s_10_3: branch s_10_2 b61 b11
        if s_10_2 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_11_0: read-var x:str
        let s_11_0: &'static str = fn_state.x;
        // C s_11_1: const #"SPE mask 10:8,4,2" : str
        let s_11_1: &'static str = "SPE mask 10:8,4,2";
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // N s_11_3: branch s_11_2 b57 b12
        if s_11_2 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_12_0: read-var x:str
        let s_12_0: &'static str = fn_state.x;
        // C s_12_1: const #"MPAM version" : str
        let s_12_1: &'static str = "MPAM version";
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // N s_12_3: branch s_12_2 b56 b13
        if s_12_2 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_13_0: read-var x:str
        let s_13_0: &'static str = fn_state.x;
        // C s_13_1: const #"MPAM maximum PARTID" : str
        let s_13_1: &'static str = "MPAM maximum PARTID";
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b55 b14
        if s_13_2 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_14_0: read-var x:str
        let s_14_0: &'static str = fn_state.x;
        // C s_14_1: const #"MPAM maximum PMG" : str
        let s_14_1: &'static str = "MPAM maximum PMG";
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // N s_14_3: branch s_14_2 b54 b15
        if s_14_2 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_15_0: read-var x:str
        let s_15_0: &'static str = fn_state.x;
        // C s_15_1: const #"Has MPAMHCR_EL2" : str
        let s_15_1: &'static str = "Has MPAMHCR_EL2";
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // N s_15_3: branch s_15_2 b50 b16
        if s_15_2 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_16_0: read-var x:str
        let s_16_0: &'static str = fn_state.x;
        // C s_16_1: const #"MPAM maximum VPMR" : str
        let s_16_1: &'static str = "MPAM maximum VPMR";
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // N s_16_3: branch s_16_2 b49 b17
        if s_16_2 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_17_0: read-var x:str
        let s_17_0: &'static str = fn_state.x;
        // C s_17_1: const #"SPE mask 63:48" : str
        let s_17_1: &'static str = "SPE mask 63:48";
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // N s_17_3: branch s_17_2 b48 b18
        if s_17_2 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_18_0: read-var x:str
        let s_18_0: &'static str = fn_state.x;
        // C s_18_1: const #"SPE mask 31:24" : str
        let s_18_1: &'static str = "SPE mask 31:24";
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // N s_18_3: branch s_18_2 b47 b19
        if s_18_2 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_19_0: read-var x:str
        let s_19_0: &'static str = fn_state.x;
        // C s_19_1: const #"SPE mask 15:12" : str
        let s_19_1: &'static str = "SPE mask 15:12";
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // N s_19_3: branch s_19_2 b46 b20
        if s_19_2 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_20_0: read-var x:str
        let s_20_0: &'static str = fn_state.x;
        // C s_20_1: const #"SPE EVENTS 63_48" : str
        let s_20_1: &'static str = "SPE EVENTS 63_48";
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // N s_20_3: branch s_20_2 b45 b21
        if s_20_2 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_21_0: read-var x:str
        let s_21_0: &'static str = fn_state.x;
        // C s_21_1: const #"SPE EVENTS 31_24" : str
        let s_21_1: &'static str = "SPE EVENTS 31_24";
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // N s_21_3: branch s_21_2 b44 b22
        if s_21_2 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_22_0: read-var x:str
        let s_22_0: &'static str = fn_state.x;
        // C s_22_1: const #"SPE EVENTS 15_12" : str
        let s_22_1: &'static str = "SPE EVENTS 15_12";
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // N s_22_3: branch s_22_2 b43 b23
        if s_22_2 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_23_0: read-var x:str
        let s_23_0: &'static str = fn_state.x;
        // C s_23_1: const #"Default Memory Map XNX bit" : str
        let s_23_1: &'static str = "Default Memory Map XNX bit";
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // N s_23_3: branch s_23_2 b42 b24
        if s_23_2 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_24_0: read-var x:str
        let s_24_0: &'static str = fn_state.x;
        // C s_24_1: const #"0" : str
        let s_24_1: &'static str = "0";
        // D s_24_2: cmp-eq s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) == (s_24_1));
        // N s_24_3: branch s_24_2 b38 b25
        if s_24_2 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_25_0: read-var x:str
        let s_25_0: &'static str = fn_state.x;
        // C s_25_1: const #"1" : str
        let s_25_1: &'static str = "1";
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // N s_25_3: branch s_25_2 b34 b26
        if s_25_2 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_26_0: read-var x:str
        let s_26_0: &'static str = fn_state.x;
        // C s_26_1: const #"IMPDEF ErrorState" : str
        let s_26_1: &'static str = "IMPDEF ErrorState";
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // N s_26_3: branch s_26_2 b33 b27
        if s_26_2 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_27_0: read-var x:str
        let s_27_0: &'static str = fn_state.x;
        // C s_27_1: const #"Virtual External abort type" : str
        let s_27_1: &'static str = "Virtual External abort type";
        // D s_27_2: cmp-eq s_27_0 s_27_1
        let s_27_2: bool = ((s_27_0) == (s_27_1));
        // N s_27_3: branch s_27_2 b32 b28
        if s_27_2 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_28_0: read-var x:str
        let s_28_0: &'static str = fn_state.x;
        // C s_28_1: const #"Virtual SError syndrome" : str
        let s_28_1: &'static str = "Virtual SError syndrome";
        // D s_28_2: cmp-eq s_28_0 s_28_1
        let s_28_2: bool = ((s_28_0) == (s_28_1));
        // N s_28_3: branch s_28_2 b31 b29
        if s_28_2 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // N s_29_1: assert s_29_0
        let s_29_1: () = assert!(s_29_0);
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_30_0: read-var return_value:bv
        let s_30_0: Bits = fn_state.return_value;
        // N s_30_1: return s_30_0
        return s_30_0;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_31_0: read-var Nshadow#8009:i
        let s_31_0: i128 = fn_state.Nshadow_8009;
        // D s_31_1: call Zeros(s_31_0)
        let s_31_1: Bits = Zeros(state, tracer, s_31_0);
        // D s_31_2: write-var return_value <= s_31_1
        fn_state.return_value = s_31_1;
        // N s_31_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_32_0: read-var Nshadow#8009:i
        let s_32_0: i128 = fn_state.Nshadow_8009;
        // D s_32_1: call Zeros(s_32_0)
        let s_32_1: Bits = Zeros(state, tracer, s_32_0);
        // D s_32_2: write-var return_value <= s_32_1
        fn_state.return_value = s_32_1;
        // N s_32_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_33_0: read-var Nshadow#8009:i
        let s_33_0: i128 = fn_state.Nshadow_8009;
        // D s_33_1: call Zeros(s_33_0)
        let s_33_1: Bits = Zeros(state, tracer, s_33_0);
        // D s_33_2: write-var return_value <= s_33_1
        fn_state.return_value = s_33_1;
        // N s_33_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_34_0: read-var Nshadow#8009:i
        let s_34_0: i128 = fn_state.Nshadow_8009;
        // D s_34_1: call __id(s_34_0)
        let s_34_1: i128 = u__id(state, tracer, s_34_0);
        // C s_34_2: const #1s : i
        let s_34_2: i128 = 1;
        // D s_34_3: sub s_34_1 s_34_2
        let s_34_3: i128 = ((s_34_1) - (s_34_2));
        // C s_34_4: const #0s : i
        let s_34_4: i128 = 0;
        // D s_34_5: cmp-le s_34_4 s_34_3
        let s_34_5: bool = ((s_34_4) <= (s_34_3));
        // N s_34_6: branch s_34_5 b37 b35
        if s_34_5 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#331391 <= s_35_0
        fn_state.gs_331391 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_36_0: read-var gs#331391:u8
        let s_36_0: bool = fn_state.gs_331391;
        // N s_36_1: assert s_36_0
        let s_36_1: () = assert!(s_36_0);
        // C s_36_2: const #0s : i
        let s_36_2: i128 = 0;
        // C s_36_3: const #1s : i
        let s_36_3: i128 = 1;
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // C s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 1u16);
        // D s_36_6: bit-extract s_36_5 s_36_2 s_36_3
        let s_36_6: Bits = (Bits::new(
            ((s_36_5) >> (s_36_2)).value(),
            u16::try_from(s_36_3).unwrap(),
        ));
        // D s_36_7: cast reint s_36_6 -> u8
        let s_36_7: bool = ((s_36_6.value()) != 0);
        // D s_36_8: cast zx s_36_7 -> bv
        let s_36_8: Bits = Bits::new(s_36_7 as u128, 1u16);
        // D s_36_9: write-var return_value <= s_36_8
        fn_state.return_value = s_36_8;
        // N s_36_10: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_37_0: read-var Nshadow#8009:i
        let s_37_0: i128 = fn_state.Nshadow_8009;
        // D s_37_1: call __id(s_37_0)
        let s_37_1: i128 = u__id(state, tracer, s_37_0);
        // C s_37_2: const #1s : i
        let s_37_2: i128 = 1;
        // D s_37_3: sub s_37_1 s_37_2
        let s_37_3: i128 = ((s_37_1) - (s_37_2));
        // C s_37_4: const #1s : i
        let s_37_4: i128 = 1;
        // D s_37_5: cmp-lt s_37_3 s_37_4
        let s_37_5: bool = ((s_37_3) < (s_37_4));
        // D s_37_6: write-var gs#331391 <= s_37_5
        fn_state.gs_331391 = s_37_5;
        // N s_37_7: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_38_0: read-var Nshadow#8009:i
        let s_38_0: i128 = fn_state.Nshadow_8009;
        // D s_38_1: call __id(s_38_0)
        let s_38_1: i128 = u__id(state, tracer, s_38_0);
        // C s_38_2: const #1s : i
        let s_38_2: i128 = 1;
        // D s_38_3: sub s_38_1 s_38_2
        let s_38_3: i128 = ((s_38_1) - (s_38_2));
        // C s_38_4: const #0s : i
        let s_38_4: i128 = 0;
        // D s_38_5: cmp-le s_38_4 s_38_3
        let s_38_5: bool = ((s_38_4) <= (s_38_3));
        // N s_38_6: branch s_38_5 b41 b39
        if s_38_5 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#331400 <= s_39_0
        fn_state.gs_331400 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_40_0: read-var gs#331400:u8
        let s_40_0: bool = fn_state.gs_331400;
        // N s_40_1: assert s_40_0
        let s_40_1: () = assert!(s_40_0);
        // C s_40_2: const #0s : i
        let s_40_2: i128 = 0;
        // C s_40_3: const #1s : i
        let s_40_3: i128 = 1;
        // C s_40_4: const #0u : u8
        let s_40_4: bool = false;
        // C s_40_5: cast zx s_40_4 -> bv
        let s_40_5: Bits = Bits::new(s_40_4 as u128, 1u16);
        // D s_40_6: bit-extract s_40_5 s_40_2 s_40_3
        let s_40_6: Bits = (Bits::new(
            ((s_40_5) >> (s_40_2)).value(),
            u16::try_from(s_40_3).unwrap(),
        ));
        // D s_40_7: cast reint s_40_6 -> u8
        let s_40_7: bool = ((s_40_6.value()) != 0);
        // D s_40_8: cast zx s_40_7 -> bv
        let s_40_8: Bits = Bits::new(s_40_7 as u128, 1u16);
        // D s_40_9: write-var return_value <= s_40_8
        fn_state.return_value = s_40_8;
        // N s_40_10: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_41_0: read-var Nshadow#8009:i
        let s_41_0: i128 = fn_state.Nshadow_8009;
        // D s_41_1: call __id(s_41_0)
        let s_41_1: i128 = u__id(state, tracer, s_41_0);
        // C s_41_2: const #1s : i
        let s_41_2: i128 = 1;
        // D s_41_3: sub s_41_1 s_41_2
        let s_41_3: i128 = ((s_41_1) - (s_41_2));
        // C s_41_4: const #1s : i
        let s_41_4: i128 = 1;
        // D s_41_5: cmp-lt s_41_3 s_41_4
        let s_41_5: bool = ((s_41_3) < (s_41_4));
        // D s_41_6: write-var gs#331400 <= s_41_5
        fn_state.gs_331400 = s_41_5;
        // N s_41_7: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_42_0: read-var Nshadow#8009:i
        let s_42_0: i128 = fn_state.Nshadow_8009;
        // D s_42_1: call __id(s_42_0)
        let s_42_1: i128 = u__id(state, tracer, s_42_0);
        // C s_42_2: const #1s : i
        let s_42_2: i128 = 1;
        // D s_42_3: sub s_42_1 s_42_2
        let s_42_3: i128 = ((s_42_1) - (s_42_2));
        // C s_42_4: const #0s : i
        let s_42_4: i128 = 0;
        // D s_42_5: cmp-le s_42_4 s_42_3
        let s_42_5: bool = ((s_42_4) <= (s_42_3));
        // N s_42_6: assert s_42_5
        let s_42_6: () = assert!(s_42_5);
        // C s_42_7: const #1s : i
        let s_42_7: i128 = 1;
        // D s_42_8: read-var Nshadow#8009:i
        let s_42_8: i128 = fn_state.Nshadow_8009;
        // D s_42_9: sub s_42_8 s_42_7
        let s_42_9: i128 = ((s_42_8) - (s_42_7));
        // C s_42_10: const #0s : i
        let s_42_10: i128 = 0;
        // C s_42_11: const #0s : i
        let s_42_11: i128 = 0;
        // D s_42_12: call integer_subrange(s_42_10, s_42_9, s_42_11)
        let s_42_12: Bits = integer_subrange(state, tracer, s_42_10, s_42_9, s_42_11);
        // D s_42_13: write-var return_value <= s_42_12
        fn_state.return_value = s_42_12;
        // N s_42_14: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_43_0: read-var Nshadow#8009:i
        let s_43_0: i128 = fn_state.Nshadow_8009;
        // D s_43_1: call Zeros(s_43_0)
        let s_43_1: Bits = Zeros(state, tracer, s_43_0);
        // D s_43_2: write-var return_value <= s_43_1
        fn_state.return_value = s_43_1;
        // N s_43_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_44_0: read-var Nshadow#8009:i
        let s_44_0: i128 = fn_state.Nshadow_8009;
        // D s_44_1: call Zeros(s_44_0)
        let s_44_1: Bits = Zeros(state, tracer, s_44_0);
        // D s_44_2: write-var return_value <= s_44_1
        fn_state.return_value = s_44_1;
        // N s_44_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_45_0: read-var Nshadow#8009:i
        let s_45_0: i128 = fn_state.Nshadow_8009;
        // D s_45_1: call Zeros(s_45_0)
        let s_45_1: Bits = Zeros(state, tracer, s_45_0);
        // D s_45_2: write-var return_value <= s_45_1
        fn_state.return_value = s_45_1;
        // N s_45_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_46_0: read-var Nshadow#8009:i
        let s_46_0: i128 = fn_state.Nshadow_8009;
        // D s_46_1: call Zeros(s_46_0)
        let s_46_1: Bits = Zeros(state, tracer, s_46_0);
        // D s_46_2: write-var return_value <= s_46_1
        fn_state.return_value = s_46_1;
        // N s_46_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_47_0: read-var Nshadow#8009:i
        let s_47_0: i128 = fn_state.Nshadow_8009;
        // D s_47_1: call Zeros(s_47_0)
        let s_47_1: Bits = Zeros(state, tracer, s_47_0);
        // D s_47_2: write-var return_value <= s_47_1
        fn_state.return_value = s_47_1;
        // N s_47_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_48_0: read-var Nshadow#8009:i
        let s_48_0: i128 = fn_state.Nshadow_8009;
        // D s_48_1: call Zeros(s_48_0)
        let s_48_1: Bits = Zeros(state, tracer, s_48_0);
        // D s_48_2: write-var return_value <= s_48_1
        fn_state.return_value = s_48_1;
        // N s_48_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_49_0: read-var Nshadow#8009:i
        let s_49_0: i128 = fn_state.Nshadow_8009;
        // D s_49_1: call __id(s_49_0)
        let s_49_1: i128 = u__id(state, tracer, s_49_0);
        // C s_49_2: const #1s : i
        let s_49_2: i128 = 1;
        // D s_49_3: sub s_49_1 s_49_2
        let s_49_3: i128 = ((s_49_1) - (s_49_2));
        // C s_49_4: const #0s : i
        let s_49_4: i128 = 0;
        // D s_49_5: cmp-le s_49_4 s_49_3
        let s_49_5: bool = ((s_49_4) <= (s_49_3));
        // N s_49_6: assert s_49_5
        let s_49_6: () = assert!(s_49_5);
        // C s_49_7: const #21192u : u32
        let s_49_7: u32 = 21192;
        // D s_49_8: read-reg s_49_7:u8
        let s_49_8: u8 = {
            let value = state.read_register::<u8>(s_49_7 as isize);
            tracer.read_register(s_49_7 as isize, value);
            value
        };
        // D s_49_9: cast zx s_49_8 -> bv
        let s_49_9: Bits = Bits::new(s_49_8 as u128, 3u16);
        // D s_49_10: cast zx s_49_9 -> i
        let s_49_10: i128 = (s_49_9.value() as i128);
        // D s_49_11: cast reint s_49_10 -> i64
        let s_49_11: i64 = (s_49_10 as i64);
        // C s_49_12: const #1s : i
        let s_49_12: i128 = 1;
        // D s_49_13: read-var Nshadow#8009:i
        let s_49_13: i128 = fn_state.Nshadow_8009;
        // D s_49_14: sub s_49_13 s_49_12
        let s_49_14: i128 = ((s_49_13) - (s_49_12));
        // C s_49_15: const #0s : i
        let s_49_15: i128 = 0;
        // D s_49_16: cast zx s_49_11 -> i
        let s_49_16: i128 = (i128::try_from(s_49_11).unwrap());
        // D s_49_17: call integer_subrange(s_49_16, s_49_14, s_49_15)
        let s_49_17: Bits = integer_subrange(state, tracer, s_49_16, s_49_14, s_49_15);
        // D s_49_18: write-var return_value <= s_49_17
        fn_state.return_value = s_49_17;
        // N s_49_19: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_50_0: const #90392u : u32
        let s_50_0: u32 = 90392;
        // D s_50_1: read-reg s_50_0:u8
        let s_50_1: bool = {
            let value = state.read_register::<bool>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // N s_50_2: branch s_50_1 b53 b51
        if s_50_1 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_51_0: read-var Nshadow#8009:i
        let s_51_0: i128 = fn_state.Nshadow_8009;
        // D s_51_1: call Zeros(s_51_0)
        let s_51_1: Bits = Zeros(state, tracer, s_51_0);
        // D s_51_2: write-var ga#370920 <= s_51_1
        fn_state.ga_370920 = s_51_1;
        // N s_51_3: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_52_0: read-var ga#370920:bv
        let s_52_0: Bits = fn_state.ga_370920;
        // D s_52_1: write-var return_value <= s_52_0
        fn_state.return_value = s_52_0;
        // N s_52_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_53_0: read-var Nshadow#8009:i
        let s_53_0: i128 = fn_state.Nshadow_8009;
        // D s_53_1: call __id(s_53_0)
        let s_53_1: i128 = u__id(state, tracer, s_53_0);
        // C s_53_2: const #1s : i
        let s_53_2: i128 = 1;
        // D s_53_3: sub s_53_1 s_53_2
        let s_53_3: i128 = ((s_53_1) - (s_53_2));
        // C s_53_4: const #0s : i
        let s_53_4: i128 = 0;
        // D s_53_5: cmp-le s_53_4 s_53_3
        let s_53_5: bool = ((s_53_4) <= (s_53_3));
        // N s_53_6: assert s_53_5
        let s_53_6: () = assert!(s_53_5);
        // C s_53_7: const #1s : i
        let s_53_7: i128 = 1;
        // D s_53_8: read-var Nshadow#8009:i
        let s_53_8: i128 = fn_state.Nshadow_8009;
        // D s_53_9: sub s_53_8 s_53_7
        let s_53_9: i128 = ((s_53_8) - (s_53_7));
        // C s_53_10: const #1s : i
        let s_53_10: i128 = 1;
        // C s_53_11: const #0s : i
        let s_53_11: i128 = 0;
        // D s_53_12: call integer_subrange(s_53_10, s_53_9, s_53_11)
        let s_53_12: Bits = integer_subrange(state, tracer, s_53_10, s_53_9, s_53_11);
        // D s_53_13: write-var ga#370920 <= s_53_12
        fn_state.ga_370920 = s_53_12;
        // N s_53_14: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_54_0: read-var Nshadow#8009:i
        let s_54_0: i128 = fn_state.Nshadow_8009;
        // D s_54_1: call __id(s_54_0)
        let s_54_1: i128 = u__id(state, tracer, s_54_0);
        // C s_54_2: const #1s : i
        let s_54_2: i128 = 1;
        // D s_54_3: sub s_54_1 s_54_2
        let s_54_3: i128 = ((s_54_1) - (s_54_2));
        // C s_54_4: const #0s : i
        let s_54_4: i128 = 0;
        // D s_54_5: cmp-le s_54_4 s_54_3
        let s_54_5: bool = ((s_54_4) <= (s_54_3));
        // N s_54_6: assert s_54_5
        let s_54_6: () = assert!(s_54_5);
        // C s_54_7: const #13392u : u32
        let s_54_7: u32 = 13392;
        // D s_54_8: read-reg s_54_7:u8
        let s_54_8: u8 = {
            let value = state.read_register::<u8>(s_54_7 as isize);
            tracer.read_register(s_54_7 as isize, value);
            value
        };
        // D s_54_9: cast zx s_54_8 -> bv
        let s_54_9: Bits = Bits::new(s_54_8 as u128, 8u16);
        // D s_54_10: cast zx s_54_9 -> i
        let s_54_10: i128 = (s_54_9.value() as i128);
        // D s_54_11: cast reint s_54_10 -> i64
        let s_54_11: i64 = (s_54_10 as i64);
        // C s_54_12: const #1s : i
        let s_54_12: i128 = 1;
        // D s_54_13: read-var Nshadow#8009:i
        let s_54_13: i128 = fn_state.Nshadow_8009;
        // D s_54_14: sub s_54_13 s_54_12
        let s_54_14: i128 = ((s_54_13) - (s_54_12));
        // C s_54_15: const #0s : i
        let s_54_15: i128 = 0;
        // D s_54_16: cast zx s_54_11 -> i
        let s_54_16: i128 = (i128::try_from(s_54_11).unwrap());
        // D s_54_17: call integer_subrange(s_54_16, s_54_14, s_54_15)
        let s_54_17: Bits = integer_subrange(state, tracer, s_54_16, s_54_14, s_54_15);
        // D s_54_18: write-var return_value <= s_54_17
        fn_state.return_value = s_54_17;
        // N s_54_19: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_55_0: read-var Nshadow#8009:i
        let s_55_0: i128 = fn_state.Nshadow_8009;
        // D s_55_1: call __id(s_55_0)
        let s_55_1: i128 = u__id(state, tracer, s_55_0);
        // C s_55_2: const #1s : i
        let s_55_2: i128 = 1;
        // D s_55_3: sub s_55_1 s_55_2
        let s_55_3: i128 = ((s_55_1) - (s_55_2));
        // C s_55_4: const #0s : i
        let s_55_4: i128 = 0;
        // D s_55_5: cmp-le s_55_4 s_55_3
        let s_55_5: bool = ((s_55_4) <= (s_55_3));
        // N s_55_6: assert s_55_5
        let s_55_6: () = assert!(s_55_5);
        // C s_55_7: const #21808u : u32
        let s_55_7: u32 = 21808;
        // D s_55_8: read-reg s_55_7:u16
        let s_55_8: u16 = {
            let value = state.read_register::<u16>(s_55_7 as isize);
            tracer.read_register(s_55_7 as isize, value);
            value
        };
        // D s_55_9: cast zx s_55_8 -> bv
        let s_55_9: Bits = Bits::new(s_55_8 as u128, 16u16);
        // D s_55_10: cast zx s_55_9 -> i
        let s_55_10: i128 = (s_55_9.value() as i128);
        // D s_55_11: cast reint s_55_10 -> i64
        let s_55_11: i64 = (s_55_10 as i64);
        // C s_55_12: const #1s : i
        let s_55_12: i128 = 1;
        // D s_55_13: read-var Nshadow#8009:i
        let s_55_13: i128 = fn_state.Nshadow_8009;
        // D s_55_14: sub s_55_13 s_55_12
        let s_55_14: i128 = ((s_55_13) - (s_55_12));
        // C s_55_15: const #0s : i
        let s_55_15: i128 = 0;
        // D s_55_16: cast zx s_55_11 -> i
        let s_55_16: i128 = (i128::try_from(s_55_11).unwrap());
        // D s_55_17: call integer_subrange(s_55_16, s_55_14, s_55_15)
        let s_55_17: Bits = integer_subrange(state, tracer, s_55_16, s_55_14, s_55_15);
        // D s_55_18: write-var return_value <= s_55_17
        fn_state.return_value = s_55_17;
        // N s_55_19: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_56_0: read-var Nshadow#8009:i
        let s_56_0: i128 = fn_state.Nshadow_8009;
        // D s_56_1: call __id(s_56_0)
        let s_56_1: i128 = u__id(state, tracer, s_56_0);
        // C s_56_2: const #1s : i
        let s_56_2: i128 = 1;
        // D s_56_3: sub s_56_1 s_56_2
        let s_56_3: i128 = ((s_56_1) - (s_56_2));
        // C s_56_4: const #0s : i
        let s_56_4: i128 = 0;
        // D s_56_5: cmp-le s_56_4 s_56_3
        let s_56_5: bool = ((s_56_4) <= (s_56_3));
        // N s_56_6: assert s_56_5
        let s_56_6: () = assert!(s_56_5);
        // C s_56_7: const #90144u : u32
        let s_56_7: u32 = 90144;
        // D s_56_8: read-reg s_56_7:u8
        let s_56_8: u8 = {
            let value = state.read_register::<u8>(s_56_7 as isize);
            tracer.read_register(s_56_7 as isize, value);
            value
        };
        // D s_56_9: cast zx s_56_8 -> bv
        let s_56_9: Bits = Bits::new(s_56_8 as u128, 4u16);
        // D s_56_10: cast zx s_56_9 -> i
        let s_56_10: i128 = (s_56_9.value() as i128);
        // D s_56_11: cast reint s_56_10 -> i64
        let s_56_11: i64 = (s_56_10 as i64);
        // C s_56_12: const #1s : i
        let s_56_12: i128 = 1;
        // D s_56_13: read-var Nshadow#8009:i
        let s_56_13: i128 = fn_state.Nshadow_8009;
        // D s_56_14: sub s_56_13 s_56_12
        let s_56_14: i128 = ((s_56_13) - (s_56_12));
        // C s_56_15: const #0s : i
        let s_56_15: i128 = 0;
        // D s_56_16: cast zx s_56_11 -> i
        let s_56_16: i128 = (i128::try_from(s_56_11).unwrap());
        // D s_56_17: call integer_subrange(s_56_16, s_56_14, s_56_15)
        let s_56_17: Bits = integer_subrange(state, tracer, s_56_16, s_56_14, s_56_15);
        // D s_56_18: write-var return_value <= s_56_17
        fn_state.return_value = s_56_17;
        // N s_56_19: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_57_0: read-var Nshadow#8009:i
        let s_57_0: i128 = fn_state.Nshadow_8009;
        // D s_57_1: call __id(s_57_0)
        let s_57_1: i128 = u__id(state, tracer, s_57_0);
        // C s_57_2: const #1s : i
        let s_57_2: i128 = 1;
        // D s_57_3: sub s_57_1 s_57_2
        let s_57_3: i128 = ((s_57_1) - (s_57_2));
        // C s_57_4: const #0s : i
        let s_57_4: i128 = 0;
        // D s_57_5: cmp-le s_57_4 s_57_3
        let s_57_5: bool = ((s_57_4) <= (s_57_3));
        // N s_57_6: branch s_57_5 b60 b58
        if s_57_5 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#331441 <= s_58_0
        fn_state.gs_331441 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_59_0: read-var gs#331441:u8
        let s_59_0: bool = fn_state.gs_331441;
        // N s_59_1: assert s_59_0
        let s_59_1: () = assert!(s_59_0);
        // C s_59_2: const #1s : i
        let s_59_2: i128 = 1;
        // D s_59_3: read-var Nshadow#8009:i
        let s_59_3: i128 = fn_state.Nshadow_8009;
        // D s_59_4: sub s_59_3 s_59_2
        let s_59_4: i128 = ((s_59_3) - (s_59_2));
        // D s_59_5: cast reint s_59_4 -> i64
        let s_59_5: i64 = (s_59_4 as i64);
        // C s_59_6: const #0s : i
        let s_59_6: i128 = 0;
        // C s_59_7: const #31u : u8
        let s_59_7: u8 = 31;
        // C s_59_8: cast zx s_59_7 -> bv
        let s_59_8: Bits = Bits::new(s_59_7 as u128, 5u16);
        // D s_59_9: cast zx s_59_5 -> i
        let s_59_9: i128 = (i128::try_from(s_59_5).unwrap());
        // C s_59_10: const #1s : i64
        let s_59_10: i64 = 1;
        // C s_59_11: cast zx s_59_10 -> i
        let s_59_11: i128 = (i128::try_from(s_59_10).unwrap());
        // D s_59_12: sub s_59_9 s_59_6
        let s_59_12: i128 = ((s_59_9) - (s_59_6));
        // D s_59_13: add s_59_12 s_59_11
        let s_59_13: i128 = (s_59_12 + s_59_11);
        // D s_59_14: bit-extract s_59_8 s_59_6 s_59_13
        let s_59_14: Bits = (Bits::new(
            ((s_59_8) >> (s_59_6)).value(),
            u16::try_from(s_59_13).unwrap(),
        ));
        // D s_59_15: write-var return_value <= s_59_14
        fn_state.return_value = s_59_14;
        // N s_59_16: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_60_0: read-var Nshadow#8009:i
        let s_60_0: i128 = fn_state.Nshadow_8009;
        // D s_60_1: call __id(s_60_0)
        let s_60_1: i128 = u__id(state, tracer, s_60_0);
        // C s_60_2: const #1s : i
        let s_60_2: i128 = 1;
        // D s_60_3: sub s_60_1 s_60_2
        let s_60_3: i128 = ((s_60_1) - (s_60_2));
        // C s_60_4: const #5s : i
        let s_60_4: i128 = 5;
        // D s_60_5: cmp-lt s_60_3 s_60_4
        let s_60_5: bool = ((s_60_3) < (s_60_4));
        // D s_60_6: write-var gs#331441 <= s_60_5
        fn_state.gs_331441 = s_60_5;
        // N s_60_7: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_61_0: read-var Nshadow#8009:i
        let s_61_0: i128 = fn_state.Nshadow_8009;
        // D s_61_1: call Zeros(s_61_0)
        let s_61_1: Bits = Zeros(state, tracer, s_61_0);
        // D s_61_2: write-var return_value <= s_61_1
        fn_state.return_value = s_61_1;
        // N s_61_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_62_0: read-var Nshadow#8009:i
        let s_62_0: i128 = fn_state.Nshadow_8009;
        // D s_62_1: call __id(s_62_0)
        let s_62_1: i128 = u__id(state, tracer, s_62_0);
        // C s_62_2: const #1s : i
        let s_62_2: i128 = 1;
        // D s_62_3: sub s_62_1 s_62_2
        let s_62_3: i128 = ((s_62_1) - (s_62_2));
        // C s_62_4: const #0s : i
        let s_62_4: i128 = 0;
        // D s_62_5: cmp-le s_62_4 s_62_3
        let s_62_5: bool = ((s_62_4) <= (s_62_3));
        // N s_62_6: branch s_62_5 b65 b63
        if s_62_5 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#331449 <= s_63_0
        fn_state.gs_331449 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_64_0: read-var gs#331449:u8
        let s_64_0: bool = fn_state.gs_331449;
        // N s_64_1: assert s_64_0
        let s_64_1: () = assert!(s_64_0);
        // C s_64_2: const #0s : i
        let s_64_2: i128 = 0;
        // C s_64_3: const #1s : i
        let s_64_3: i128 = 1;
        // C s_64_4: const #0u : u8
        let s_64_4: bool = false;
        // C s_64_5: cast zx s_64_4 -> bv
        let s_64_5: Bits = Bits::new(s_64_4 as u128, 1u16);
        // D s_64_6: bit-extract s_64_5 s_64_2 s_64_3
        let s_64_6: Bits = (Bits::new(
            ((s_64_5) >> (s_64_2)).value(),
            u16::try_from(s_64_3).unwrap(),
        ));
        // D s_64_7: cast reint s_64_6 -> u8
        let s_64_7: bool = ((s_64_6.value()) != 0);
        // D s_64_8: cast zx s_64_7 -> bv
        let s_64_8: Bits = Bits::new(s_64_7 as u128, 1u16);
        // D s_64_9: write-var return_value <= s_64_8
        fn_state.return_value = s_64_8;
        // N s_64_10: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_65_0: read-var Nshadow#8009:i
        let s_65_0: i128 = fn_state.Nshadow_8009;
        // D s_65_1: call __id(s_65_0)
        let s_65_1: i128 = u__id(state, tracer, s_65_0);
        // C s_65_2: const #1s : i
        let s_65_2: i128 = 1;
        // D s_65_3: sub s_65_1 s_65_2
        let s_65_3: i128 = ((s_65_1) - (s_65_2));
        // C s_65_4: const #1s : i
        let s_65_4: i128 = 1;
        // D s_65_5: cmp-lt s_65_3 s_65_4
        let s_65_5: bool = ((s_65_3) < (s_65_4));
        // D s_65_6: write-var gs#331449 <= s_65_5
        fn_state.gs_331449 = s_65_5;
        // N s_65_7: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_66_0: read-var Nshadow#8009:i
        let s_66_0: i128 = fn_state.Nshadow_8009;
        // D s_66_1: call __id(s_66_0)
        let s_66_1: i128 = u__id(state, tracer, s_66_0);
        // C s_66_2: const #1s : i
        let s_66_2: i128 = 1;
        // D s_66_3: sub s_66_1 s_66_2
        let s_66_3: i128 = ((s_66_1) - (s_66_2));
        // C s_66_4: const #0s : i
        let s_66_4: i128 = 0;
        // D s_66_5: cmp-le s_66_4 s_66_3
        let s_66_5: bool = ((s_66_4) <= (s_66_3));
        // N s_66_6: assert s_66_5
        let s_66_6: () = assert!(s_66_5);
        // C s_66_7: const #5u : u8
        let s_66_7: u8 = 5;
        // C s_66_8: cast zx s_66_7 -> bv
        let s_66_8: Bits = Bits::new(s_66_7 as u128, 3u16);
        // C s_66_9: cast zx s_66_8 -> i
        let s_66_9: i128 = (s_66_8.value() as i128);
        // C s_66_10: cast reint s_66_9 -> i64
        let s_66_10: i64 = (s_66_9 as i64);
        // C s_66_11: const #1s : i
        let s_66_11: i128 = 1;
        // D s_66_12: read-var Nshadow#8009:i
        let s_66_12: i128 = fn_state.Nshadow_8009;
        // D s_66_13: sub s_66_12 s_66_11
        let s_66_13: i128 = ((s_66_12) - (s_66_11));
        // C s_66_14: const #0s : i
        let s_66_14: i128 = 0;
        // C s_66_15: cast zx s_66_10 -> i
        let s_66_15: i128 = (i128::try_from(s_66_10).unwrap());
        // D s_66_16: call integer_subrange(s_66_15, s_66_13, s_66_14)
        let s_66_16: Bits = integer_subrange(state, tracer, s_66_15, s_66_13, s_66_14);
        // D s_66_17: write-var return_value <= s_66_16
        fn_state.return_value = s_66_16;
        // N s_66_18: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_67_0: read-var Nshadow#8009:i
        let s_67_0: i128 = fn_state.Nshadow_8009;
        // D s_67_1: call __id(s_67_0)
        let s_67_1: i128 = u__id(state, tracer, s_67_0);
        // C s_67_2: const #1s : i
        let s_67_2: i128 = 1;
        // D s_67_3: sub s_67_1 s_67_2
        let s_67_3: i128 = ((s_67_1) - (s_67_2));
        // C s_67_4: const #0s : i
        let s_67_4: i128 = 0;
        // D s_67_5: cmp-le s_67_4 s_67_3
        let s_67_5: bool = ((s_67_4) <= (s_67_3));
        // N s_67_6: branch s_67_5 b70 b68
        if s_67_5 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var gs#331463 <= s_68_0
        fn_state.gs_331463 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_69_0: read-var gs#331463:u8
        let s_69_0: bool = fn_state.gs_331463;
        // N s_69_1: assert s_69_0
        let s_69_1: () = assert!(s_69_0);
        // C s_69_2: const #1s : i
        let s_69_2: i128 = 1;
        // D s_69_3: read-var Nshadow#8009:i
        let s_69_3: i128 = fn_state.Nshadow_8009;
        // D s_69_4: sub s_69_3 s_69_2
        let s_69_4: i128 = ((s_69_3) - (s_69_2));
        // D s_69_5: cast reint s_69_4 -> i64
        let s_69_5: i64 = (s_69_4 as i64);
        // C s_69_6: const #0s : i
        let s_69_6: i128 = 0;
        // C s_69_7: const #13728u : u32
        let s_69_7: u32 = 13728;
        // D s_69_8: read-reg s_69_7:u8
        let s_69_8: u8 = {
            let value = state.read_register::<u8>(s_69_7 as isize);
            tracer.read_register(s_69_7 as isize, value);
            value
        };
        // D s_69_9: cast zx s_69_8 -> bv
        let s_69_9: Bits = Bits::new(s_69_8 as u128, 2u16);
        // D s_69_10: cast zx s_69_5 -> i
        let s_69_10: i128 = (i128::try_from(s_69_5).unwrap());
        // C s_69_11: const #1s : i64
        let s_69_11: i64 = 1;
        // C s_69_12: cast zx s_69_11 -> i
        let s_69_12: i128 = (i128::try_from(s_69_11).unwrap());
        // D s_69_13: sub s_69_10 s_69_6
        let s_69_13: i128 = ((s_69_10) - (s_69_6));
        // D s_69_14: add s_69_13 s_69_12
        let s_69_14: i128 = (s_69_13 + s_69_12);
        // D s_69_15: bit-extract s_69_9 s_69_6 s_69_14
        let s_69_15: Bits = (Bits::new(
            ((s_69_9) >> (s_69_6)).value(),
            u16::try_from(s_69_14).unwrap(),
        ));
        // D s_69_16: write-var return_value <= s_69_15
        fn_state.return_value = s_69_15;
        // N s_69_17: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_70_0: read-var Nshadow#8009:i
        let s_70_0: i128 = fn_state.Nshadow_8009;
        // D s_70_1: call __id(s_70_0)
        let s_70_1: i128 = u__id(state, tracer, s_70_0);
        // C s_70_2: const #1s : i
        let s_70_2: i128 = 1;
        // D s_70_3: sub s_70_1 s_70_2
        let s_70_3: i128 = ((s_70_1) - (s_70_2));
        // C s_70_4: const #2s : i
        let s_70_4: i128 = 2;
        // D s_70_5: cmp-lt s_70_3 s_70_4
        let s_70_5: bool = ((s_70_3) < (s_70_4));
        // D s_70_6: write-var gs#331463 <= s_70_5
        fn_state.gs_331463 = s_70_5;
        // N s_70_7: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_71_0: read-var Nshadow#8009:i
        let s_71_0: i128 = fn_state.Nshadow_8009;
        // D s_71_1: call __id(s_71_0)
        let s_71_1: i128 = u__id(state, tracer, s_71_0);
        // C s_71_2: const #1s : i
        let s_71_2: i128 = 1;
        // D s_71_3: sub s_71_1 s_71_2
        let s_71_3: i128 = ((s_71_1) - (s_71_2));
        // C s_71_4: const #0s : i
        let s_71_4: i128 = 0;
        // D s_71_5: cmp-le s_71_4 s_71_3
        let s_71_5: bool = ((s_71_4) <= (s_71_3));
        // N s_71_6: branch s_71_5 b74 b72
        if s_71_5 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // D s_72_1: write-var gs#331471 <= s_72_0
        fn_state.gs_331471 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_73_0: read-var gs#331471:u8
        let s_73_0: bool = fn_state.gs_331471;
        // N s_73_1: assert s_73_0
        let s_73_1: () = assert!(s_73_0);
        // C s_73_2: const #1s : i
        let s_73_2: i128 = 1;
        // D s_73_3: read-var Nshadow#8009:i
        let s_73_3: i128 = fn_state.Nshadow_8009;
        // D s_73_4: sub s_73_3 s_73_2
        let s_73_4: i128 = ((s_73_3) - (s_73_2));
        // D s_73_5: cast reint s_73_4 -> i64
        let s_73_5: i64 = (s_73_4 as i64);
        // C s_73_6: const #0s : i
        let s_73_6: i128 = 0;
        // C s_73_7: const #16544u : u32
        let s_73_7: u32 = 16544;
        // D s_73_8: read-reg s_73_7:u8
        let s_73_8: u8 = {
            let value = state.read_register::<u8>(s_73_7 as isize);
            tracer.read_register(s_73_7 as isize, value);
            value
        };
        // D s_73_9: cast zx s_73_8 -> bv
        let s_73_9: Bits = Bits::new(s_73_8 as u128, 2u16);
        // D s_73_10: cast zx s_73_5 -> i
        let s_73_10: i128 = (i128::try_from(s_73_5).unwrap());
        // C s_73_11: const #1s : i64
        let s_73_11: i64 = 1;
        // C s_73_12: cast zx s_73_11 -> i
        let s_73_12: i128 = (i128::try_from(s_73_11).unwrap());
        // D s_73_13: sub s_73_10 s_73_6
        let s_73_13: i128 = ((s_73_10) - (s_73_6));
        // D s_73_14: add s_73_13 s_73_12
        let s_73_14: i128 = (s_73_13 + s_73_12);
        // D s_73_15: bit-extract s_73_9 s_73_6 s_73_14
        let s_73_15: Bits = (Bits::new(
            ((s_73_9) >> (s_73_6)).value(),
            u16::try_from(s_73_14).unwrap(),
        ));
        // D s_73_16: write-var return_value <= s_73_15
        fn_state.return_value = s_73_15;
        // N s_73_17: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_74_0: read-var Nshadow#8009:i
        let s_74_0: i128 = fn_state.Nshadow_8009;
        // D s_74_1: call __id(s_74_0)
        let s_74_1: i128 = u__id(state, tracer, s_74_0);
        // C s_74_2: const #1s : i
        let s_74_2: i128 = 1;
        // D s_74_3: sub s_74_1 s_74_2
        let s_74_3: i128 = ((s_74_1) - (s_74_2));
        // C s_74_4: const #2s : i
        let s_74_4: i128 = 2;
        // D s_74_5: cmp-lt s_74_3 s_74_4
        let s_74_5: bool = ((s_74_3) < (s_74_4));
        // D s_74_6: write-var gs#331471 <= s_74_5
        fn_state.gs_331471 = s_74_5;
        // N s_74_7: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_75_0: read-var Nshadow#8009:i
        let s_75_0: i128 = fn_state.Nshadow_8009;
        // D s_75_1: call __id(s_75_0)
        let s_75_1: i128 = u__id(state, tracer, s_75_0);
        // C s_75_2: const #1s : i
        let s_75_2: i128 = 1;
        // D s_75_3: sub s_75_1 s_75_2
        let s_75_3: i128 = ((s_75_1) - (s_75_2));
        // C s_75_4: const #0s : i
        let s_75_4: i128 = 0;
        // D s_75_5: cmp-le s_75_4 s_75_3
        let s_75_5: bool = ((s_75_4) <= (s_75_3));
        // N s_75_6: branch s_75_5 b78 b76
        if s_75_5 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var gs#331479 <= s_76_0
        fn_state.gs_331479 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_77_0: read-var gs#331479:u8
        let s_77_0: bool = fn_state.gs_331479;
        // N s_77_1: assert s_77_0
        let s_77_1: () = assert!(s_77_0);
        // C s_77_2: const #1s : i
        let s_77_2: i128 = 1;
        // D s_77_3: read-var Nshadow#8009:i
        let s_77_3: i128 = fn_state.Nshadow_8009;
        // D s_77_4: sub s_77_3 s_77_2
        let s_77_4: i128 = ((s_77_3) - (s_77_2));
        // D s_77_5: cast reint s_77_4 -> i64
        let s_77_5: i64 = (s_77_4 as i64);
        // C s_77_6: const #0s : i
        let s_77_6: i128 = 0;
        // C s_77_7: const #102440u : u32
        let s_77_7: u32 = 102440;
        // D s_77_8: read-reg s_77_7:u64
        let s_77_8: u64 = {
            let value = state.read_register::<u64>(s_77_7 as isize);
            tracer.read_register(s_77_7 as isize, value);
            value
        };
        // D s_77_9: cast zx s_77_8 -> bv
        let s_77_9: Bits = Bits::new(s_77_8 as u128, 64u16);
        // D s_77_10: cast zx s_77_5 -> i
        let s_77_10: i128 = (i128::try_from(s_77_5).unwrap());
        // C s_77_11: const #1s : i64
        let s_77_11: i64 = 1;
        // C s_77_12: cast zx s_77_11 -> i
        let s_77_12: i128 = (i128::try_from(s_77_11).unwrap());
        // D s_77_13: sub s_77_10 s_77_6
        let s_77_13: i128 = ((s_77_10) - (s_77_6));
        // D s_77_14: add s_77_13 s_77_12
        let s_77_14: i128 = (s_77_13 + s_77_12);
        // D s_77_15: bit-extract s_77_9 s_77_6 s_77_14
        let s_77_15: Bits = (Bits::new(
            ((s_77_9) >> (s_77_6)).value(),
            u16::try_from(s_77_14).unwrap(),
        ));
        // D s_77_16: write-var return_value <= s_77_15
        fn_state.return_value = s_77_15;
        // N s_77_17: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_78_0: read-var Nshadow#8009:i
        let s_78_0: i128 = fn_state.Nshadow_8009;
        // D s_78_1: call __id(s_78_0)
        let s_78_1: i128 = u__id(state, tracer, s_78_0);
        // C s_78_2: const #1s : i
        let s_78_2: i128 = 1;
        // D s_78_3: sub s_78_1 s_78_2
        let s_78_3: i128 = ((s_78_1) - (s_78_2));
        // C s_78_4: const #64s : i
        let s_78_4: i128 = 64;
        // D s_78_5: cmp-lt s_78_3 s_78_4
        let s_78_5: bool = ((s_78_3) < (s_78_4));
        // D s_78_6: write-var gs#331479 <= s_78_5
        fn_state.gs_331479 = s_78_5;
        // N s_78_7: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_79_0: read-var Nshadow#8009:i
        let s_79_0: i128 = fn_state.Nshadow_8009;
        // D s_79_1: call __id(s_79_0)
        let s_79_1: i128 = u__id(state, tracer, s_79_0);
        // C s_79_2: const #1s : i
        let s_79_2: i128 = 1;
        // D s_79_3: sub s_79_1 s_79_2
        let s_79_3: i128 = ((s_79_1) - (s_79_2));
        // C s_79_4: const #0s : i
        let s_79_4: i128 = 0;
        // D s_79_5: cmp-le s_79_4 s_79_3
        let s_79_5: bool = ((s_79_4) <= (s_79_3));
        // N s_79_6: assert s_79_5
        let s_79_6: () = assert!(s_79_5);
        // C s_79_7: const #1s : i
        let s_79_7: i128 = 1;
        // D s_79_8: read-var Nshadow#8009:i
        let s_79_8: i128 = fn_state.Nshadow_8009;
        // D s_79_9: sub s_79_8 s_79_7
        let s_79_9: i128 = ((s_79_8) - (s_79_7));
        // C s_79_10: const #1s : i
        let s_79_10: i128 = 1;
        // C s_79_11: const #0s : i
        let s_79_11: i128 = 0;
        // D s_79_12: call integer_subrange(s_79_10, s_79_9, s_79_11)
        let s_79_12: Bits = integer_subrange(state, tracer, s_79_10, s_79_9, s_79_11);
        // D s_79_13: write-var return_value <= s_79_12
        fn_state.return_value = s_79_12;
        // N s_79_14: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_80_0: read-var Nshadow#8009:i
        let s_80_0: i128 = fn_state.Nshadow_8009;
        // D s_80_1: call Zeros(s_80_0)
        let s_80_1: Bits = Zeros(state, tracer, s_80_0);
        // D s_80_2: write-var return_value <= s_80_1
        fn_state.return_value = s_80_1;
        // N s_80_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_81_0: read-var Nshadow#8009:i
        let s_81_0: i128 = fn_state.Nshadow_8009;
        // D s_81_1: call __id(s_81_0)
        let s_81_1: i128 = u__id(state, tracer, s_81_0);
        // C s_81_2: const #1s : i
        let s_81_2: i128 = 1;
        // D s_81_3: sub s_81_1 s_81_2
        let s_81_3: i128 = ((s_81_1) - (s_81_2));
        // C s_81_4: const #0s : i
        let s_81_4: i128 = 0;
        // D s_81_5: cmp-le s_81_4 s_81_3
        let s_81_5: bool = ((s_81_4) <= (s_81_3));
        // N s_81_6: branch s_81_5 b84 b82
        if s_81_5 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_82_0: const #0u : u8
        let s_82_0: bool = false;
        // D s_82_1: write-var gs#331493 <= s_82_0
        fn_state.gs_331493 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_83_0: read-var gs#331493:u8
        let s_83_0: bool = fn_state.gs_331493;
        // N s_83_1: assert s_83_0
        let s_83_1: () = assert!(s_83_0);
        // C s_83_2: const #1s : i
        let s_83_2: i128 = 1;
        // D s_83_3: read-var Nshadow#8009:i
        let s_83_3: i128 = fn_state.Nshadow_8009;
        // D s_83_4: sub s_83_3 s_83_2
        let s_83_4: i128 = ((s_83_3) - (s_83_2));
        // D s_83_5: cast reint s_83_4 -> i64
        let s_83_5: i64 = (s_83_4 as i64);
        // C s_83_6: const #0s : i
        let s_83_6: i128 = 0;
        // C s_83_7: const #3u : u8
        let s_83_7: u8 = 3;
        // C s_83_8: cast zx s_83_7 -> bv
        let s_83_8: Bits = Bits::new(s_83_7 as u128, 2u16);
        // D s_83_9: cast zx s_83_5 -> i
        let s_83_9: i128 = (i128::try_from(s_83_5).unwrap());
        // C s_83_10: const #1s : i64
        let s_83_10: i64 = 1;
        // C s_83_11: cast zx s_83_10 -> i
        let s_83_11: i128 = (i128::try_from(s_83_10).unwrap());
        // D s_83_12: sub s_83_9 s_83_6
        let s_83_12: i128 = ((s_83_9) - (s_83_6));
        // D s_83_13: add s_83_12 s_83_11
        let s_83_13: i128 = (s_83_12 + s_83_11);
        // D s_83_14: bit-extract s_83_8 s_83_6 s_83_13
        let s_83_14: Bits = (Bits::new(
            ((s_83_8) >> (s_83_6)).value(),
            u16::try_from(s_83_13).unwrap(),
        ));
        // D s_83_15: write-var return_value <= s_83_14
        fn_state.return_value = s_83_14;
        // N s_83_16: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_84_0: read-var Nshadow#8009:i
        let s_84_0: i128 = fn_state.Nshadow_8009;
        // D s_84_1: call __id(s_84_0)
        let s_84_1: i128 = u__id(state, tracer, s_84_0);
        // C s_84_2: const #1s : i
        let s_84_2: i128 = 1;
        // D s_84_3: sub s_84_1 s_84_2
        let s_84_3: i128 = ((s_84_1) - (s_84_2));
        // C s_84_4: const #2s : i
        let s_84_4: i128 = 2;
        // D s_84_5: cmp-lt s_84_3 s_84_4
        let s_84_5: bool = ((s_84_3) < (s_84_4));
        // D s_84_6: write-var gs#331493 <= s_84_5
        fn_state.gs_331493 = s_84_5;
        // N s_84_7: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_85_0: read-var Nshadow#8009:i
        let s_85_0: i128 = fn_state.Nshadow_8009;
        // D s_85_1: call __id(s_85_0)
        let s_85_1: i128 = u__id(state, tracer, s_85_0);
        // C s_85_2: const #1s : i
        let s_85_2: i128 = 1;
        // D s_85_3: sub s_85_1 s_85_2
        let s_85_3: i128 = ((s_85_1) - (s_85_2));
        // C s_85_4: const #0s : i
        let s_85_4: i128 = 0;
        // D s_85_5: cmp-le s_85_4 s_85_3
        let s_85_5: bool = ((s_85_4) <= (s_85_3));
        // N s_85_6: branch s_85_5 b88 b86
        if s_85_5 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#331501 <= s_86_0
        fn_state.gs_331501 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_87_0: read-var gs#331501:u8
        let s_87_0: bool = fn_state.gs_331501;
        // N s_87_1: assert s_87_0
        let s_87_1: () = assert!(s_87_0);
        // C s_87_2: const #1s : i
        let s_87_2: i128 = 1;
        // D s_87_3: read-var Nshadow#8009:i
        let s_87_3: i128 = fn_state.Nshadow_8009;
        // D s_87_4: sub s_87_3 s_87_2
        let s_87_4: i128 = ((s_87_3) - (s_87_2));
        // D s_87_5: cast reint s_87_4 -> i64
        let s_87_5: i64 = (s_87_4 as i64);
        // C s_87_6: const #0s : i
        let s_87_6: i128 = 0;
        // C s_87_7: const #0u : u8
        let s_87_7: u8 = 0;
        // C s_87_8: cast zx s_87_7 -> bv
        let s_87_8: Bits = Bits::new(s_87_7 as u128, 2u16);
        // D s_87_9: cast zx s_87_5 -> i
        let s_87_9: i128 = (i128::try_from(s_87_5).unwrap());
        // C s_87_10: const #1s : i64
        let s_87_10: i64 = 1;
        // C s_87_11: cast zx s_87_10 -> i
        let s_87_11: i128 = (i128::try_from(s_87_10).unwrap());
        // D s_87_12: sub s_87_9 s_87_6
        let s_87_12: i128 = ((s_87_9) - (s_87_6));
        // D s_87_13: add s_87_12 s_87_11
        let s_87_13: i128 = (s_87_12 + s_87_11);
        // D s_87_14: bit-extract s_87_8 s_87_6 s_87_13
        let s_87_14: Bits = (Bits::new(
            ((s_87_8) >> (s_87_6)).value(),
            u16::try_from(s_87_13).unwrap(),
        ));
        // D s_87_15: write-var return_value <= s_87_14
        fn_state.return_value = s_87_14;
        // N s_87_16: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_88_0: read-var Nshadow#8009:i
        let s_88_0: i128 = fn_state.Nshadow_8009;
        // D s_88_1: call __id(s_88_0)
        let s_88_1: i128 = u__id(state, tracer, s_88_0);
        // C s_88_2: const #1s : i
        let s_88_2: i128 = 1;
        // D s_88_3: sub s_88_1 s_88_2
        let s_88_3: i128 = ((s_88_1) - (s_88_2));
        // C s_88_4: const #2s : i
        let s_88_4: i128 = 2;
        // D s_88_5: cmp-lt s_88_3 s_88_4
        let s_88_5: bool = ((s_88_3) < (s_88_4));
        // D s_88_6: write-var gs#331501 <= s_88_5
        fn_state.gs_331501 = s_88_5;
        // N s_88_7: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_89_0: read-var Nshadow#8009:i
        let s_89_0: i128 = fn_state.Nshadow_8009;
        // D s_89_1: call __id(s_89_0)
        let s_89_1: i128 = u__id(state, tracer, s_89_0);
        // C s_89_2: const #1s : i
        let s_89_2: i128 = 1;
        // D s_89_3: sub s_89_1 s_89_2
        let s_89_3: i128 = ((s_89_1) - (s_89_2));
        // C s_89_4: const #0s : i
        let s_89_4: i128 = 0;
        // D s_89_5: cmp-le s_89_4 s_89_3
        let s_89_5: bool = ((s_89_4) <= (s_89_3));
        // N s_89_6: branch s_89_5 b92 b90
        if s_89_5 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#331509 <= s_90_0
        fn_state.gs_331509 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_91_0: read-var gs#331509:u8
        let s_91_0: bool = fn_state.gs_331509;
        // N s_91_1: assert s_91_0
        let s_91_1: () = assert!(s_91_0);
        // C s_91_2: const #() : ()
        let s_91_2: () = ();
        // S s_91_3: call PhysicalCountInt(s_91_2)
        let s_91_3: u64 = PhysicalCountInt(state, tracer, s_91_2);
        // C s_91_4: const #1s : i
        let s_91_4: i128 = 1;
        // D s_91_5: read-var Nshadow#8009:i
        let s_91_5: i128 = fn_state.Nshadow_8009;
        // D s_91_6: sub s_91_5 s_91_4
        let s_91_6: i128 = ((s_91_5) - (s_91_4));
        // D s_91_7: cast reint s_91_6 -> i64
        let s_91_7: i64 = (s_91_6 as i64);
        // C s_91_8: const #0s : i
        let s_91_8: i128 = 0;
        // S s_91_9: cast zx s_91_3 -> bv
        let s_91_9: Bits = Bits::new(s_91_3 as u128, 64u16);
        // D s_91_10: cast zx s_91_7 -> i
        let s_91_10: i128 = (i128::try_from(s_91_7).unwrap());
        // C s_91_11: const #1s : i64
        let s_91_11: i64 = 1;
        // C s_91_12: cast zx s_91_11 -> i
        let s_91_12: i128 = (i128::try_from(s_91_11).unwrap());
        // D s_91_13: sub s_91_10 s_91_8
        let s_91_13: i128 = ((s_91_10) - (s_91_8));
        // D s_91_14: add s_91_13 s_91_12
        let s_91_14: i128 = (s_91_13 + s_91_12);
        // D s_91_15: bit-extract s_91_9 s_91_8 s_91_14
        let s_91_15: Bits = (Bits::new(
            ((s_91_9) >> (s_91_8)).value(),
            u16::try_from(s_91_14).unwrap(),
        ));
        // D s_91_16: write-var return_value <= s_91_15
        fn_state.return_value = s_91_15;
        // N s_91_17: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_92_0: read-var Nshadow#8009:i
        let s_92_0: i128 = fn_state.Nshadow_8009;
        // D s_92_1: call __id(s_92_0)
        let s_92_1: i128 = u__id(state, tracer, s_92_0);
        // C s_92_2: const #1s : i
        let s_92_2: i128 = 1;
        // D s_92_3: sub s_92_1 s_92_2
        let s_92_3: i128 = ((s_92_1) - (s_92_2));
        // C s_92_4: const #64s : i
        let s_92_4: i128 = 64;
        // D s_92_5: cmp-lt s_92_3 s_92_4
        let s_92_5: bool = ((s_92_3) < (s_92_4));
        // D s_92_6: write-var gs#331509 <= s_92_5
        fn_state.gs_331509 = s_92_5;
        // N s_92_7: jump b91
        return block_91(state, tracer, fn_state);
    }
}
