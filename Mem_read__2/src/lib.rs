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
use AArch64_UnalignedAccessFaults::*;
use u__id::*;
use HaveLSE2Ext::*;
use AllInAlignedQuantity::*;
use AArch64_MemSingle_read__1::*;
use BigEndian::*;
use IsAligned__1::*;
use HaveLRCPC3Ext::*;
use AArch64_Abort::*;
use AArch64_MemSingle_read::*;
use ConstrainUnpredictable::*;
use reverse_endianness::*;
use AlignmentFault::*;
use common::*;
pub fn Mem_read__2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    size: i64,
    accdesc: ProductType9878976b5bcce9c9,
    ispair: bool,
    highestAddressfirst: bool,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        gs_20343: bool,
        ga_15755: i64,
        gs_20329: i64,
        cshadow_328: u32,
        value_name: Bits,
        u_926: i64,
        u_924: Bits,
        ga_15756: i64,
        halfsize: i64,
        gs_20295: bool,
        gs_20290: bool,
        gs_20292: bool,
        ga_15750: i64,
        gs_20284: bool,
        u_927: i64,
        gs_20283: bool,
        gs_20275: bool,
        u_922: Bits,
        u_925: Bits,
        i: i64,
        gs_20296: bool,
        gs_447078: Bits,
        gs_20318: i64,
        gs_20301: bool,
        gs_20345: bool,
        gs_20341: bool,
        gs_20279: bool,
        gs_447043: Bits,
        gs_20288: bool,
        highhalf: Bits,
        gs_447032: Bits,
        ga_15711: i64,
        u_923: Bits,
        ga_15751: i64,
        gs_447065: Bits,
        gs_20347: bool,
        aligned: bool,
        lowhalf: Bits,
        gs_20307: i64,
        ga_15765: i64,
        gs_20277: bool,
        ga_15764: i64,
        address: u64,
        size: i64,
        accdesc: ProductType9878976b5bcce9c9,
        ispair: bool,
        highestAddressfirst: bool,
    }
    let fn_state = FunctionState {
        address,
        size,
        accdesc,
        ispair,
        highestAddressfirst,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_0_0: const #2s : i
        let s_0_0: i128 = 2;
        // D s_0_1: read-var size:i64
        let s_0_1: i64 = fn_state.size;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: div s_0_2 s_0_0
        let s_0_3: i128 = ((s_0_2) / (s_0_0));
        // D s_0_4: cast reint s_0_3 -> i64
        let s_0_4: i64 = (s_0_3 as i64);
        // D s_0_5: write-var halfsize <= s_0_4
        fn_state.halfsize = s_0_4;
        // D s_0_6: read-var ispair:u8
        let s_0_6: bool = fn_state.ispair;
        // N s_0_7: branch s_0_6 b92 b1
        if s_0_6 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_1_0: read-var size:i64
        let s_1_0: i64 = fn_state.size;
        // D s_1_1: write-var ga#15711 <= s_1_0
        fn_state.ga_15711 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var ga#15711:i64
        let s_2_0: i64 = fn_state.ga_15711;
        // D s_2_1: read-var address:u64
        let s_2_1: u64 = fn_state.address;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 64u16);
        // D s_2_3: cast zx s_2_0 -> i
        let s_2_3: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_4: call IsAligned__1(s_2_2, s_2_3)
        let s_2_4: bool = IsAligned__1(state, tracer, s_2_2, s_2_3);
        // D s_2_5: write-var aligned <= s_2_4
        fn_state.aligned = s_2_4;
        // D s_2_6: read-var aligned:u8
        let s_2_6: bool = fn_state.aligned;
        // D s_2_7: not s_2_6
        let s_2_7: bool = !s_2_6;
        // N s_2_8: branch s_2_7 b91 b3
        if s_2_7 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#20275 <= s_3_0
        fn_state.gs_20275 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_4_0: read-var gs#20275:u8
        let s_4_0: bool = fn_state.gs_20275;
        // N s_4_1: branch s_4_0 b90 b5
        if s_4_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_6_0: read-var accdesc.1:struct
        let s_6_0: u32 = fn_state.accdesc._1;
        // C s_6_1: const #2u : u32
        let s_6_1: u32 = 2;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // N s_6_3: branch s_6_2 b89 b7
        if s_6_2 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#20277 <= s_7_0
        fn_state.gs_20277 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var gs#20277:u8
        let s_8_0: bool = fn_state.gs_20277;
        // N s_8_1: branch s_8_0 b88 b9
        if s_8_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#20279 <= s_9_0
        fn_state.gs_20279 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_10_0: read-var gs#20279:u8
        let s_10_0: bool = fn_state.gs_20279;
        // N s_10_1: branch s_10_0 b85 b11
        if s_10_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call HaveLSE2Ext(s_11_0)
        let s_11_1: bool = HaveLSE2Ext(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b84 b12
        if s_11_1 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#20283 <= s_12_0
        fn_state.gs_20283 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_13_0: read-var gs#20283:u8
        let s_13_0: bool = fn_state.gs_20283;
        // N s_13_1: branch s_13_0 b82 b14
        if s_13_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_14_0: read-var ispair:u8
        let s_14_0: bool = fn_state.ispair;
        // N s_14_1: branch s_14_0 b81 b15
        if s_14_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#20284 <= s_15_0
        fn_state.gs_20284 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_16_0: read-var gs#20284:u8
        let s_16_0: bool = fn_state.gs_20284;
        // N s_16_1: branch s_16_0 b61 b17
        if s_16_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_17_0: read-var aligned:u8
        let s_17_0: bool = fn_state.aligned;
        // N s_17_1: branch s_17_0 b59 b18
        if s_17_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_18_0: const #1s : i
        let s_18_0: i128 = 1;
        // D s_18_1: read-var size:i64
        let s_18_1: i64 = fn_state.size;
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (i128::try_from(s_18_1).unwrap());
        // D s_18_3: cmp-gt s_18_2 s_18_0
        let s_18_3: bool = ((s_18_2) > (s_18_0));
        // N s_18_4: assert s_18_3
        let s_18_4: () = assert!(s_18_3);
        // D s_18_5: read-var halfsize:i64
        let s_18_5: i64 = fn_state.halfsize;
        // D s_18_6: cast zx s_18_5 -> i
        let s_18_6: i128 = (i128::try_from(s_18_5).unwrap());
        // D s_18_7: call __id(s_18_6)
        let s_18_7: i128 = u__id(state, tracer, s_18_6);
        // D s_18_8: cast reint s_18_7 -> i64
        let s_18_8: i64 = (s_18_7 as i64);
        // C s_18_9: const #1s : i
        let s_18_9: i128 = 1;
        // D s_18_10: cast zx s_18_8 -> i
        let s_18_10: i128 = (i128::try_from(s_18_8).unwrap());
        // D s_18_11: cmp-eq s_18_10 s_18_9
        let s_18_11: bool = ((s_18_10) == (s_18_9));
        // N s_18_12: branch s_18_11 b58 b19
        if s_18_11 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_19_0: read-var halfsize:i64
        let s_19_0: i64 = fn_state.halfsize;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: call __id(s_19_1)
        let s_19_2: i128 = u__id(state, tracer, s_19_1);
        // D s_19_3: cast reint s_19_2 -> i64
        let s_19_3: i64 = (s_19_2 as i64);
        // C s_19_4: const #2s : i
        let s_19_4: i128 = 2;
        // D s_19_5: cast zx s_19_3 -> i
        let s_19_5: i128 = (i128::try_from(s_19_3).unwrap());
        // D s_19_6: cmp-eq s_19_5 s_19_4
        let s_19_6: bool = ((s_19_5) == (s_19_4));
        // D s_19_7: write-var gs#20288 <= s_19_6
        fn_state.gs_20288 = s_19_6;
        // N s_19_8: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_20_0: read-var gs#20288:u8
        let s_20_0: bool = fn_state.gs_20288;
        // N s_20_1: branch s_20_0 b57 b21
        if s_20_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_21_0: read-var halfsize:i64
        let s_21_0: i64 = fn_state.halfsize;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: call __id(s_21_1)
        let s_21_2: i128 = u__id(state, tracer, s_21_1);
        // D s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // C s_21_4: const #4s : i
        let s_21_4: i128 = 4;
        // D s_21_5: cast zx s_21_3 -> i
        let s_21_5: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_6: cmp-eq s_21_5 s_21_4
        let s_21_6: bool = ((s_21_5) == (s_21_4));
        // D s_21_7: write-var gs#20290 <= s_21_6
        fn_state.gs_20290 = s_21_6;
        // N s_21_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_22_0: read-var gs#20290:u8
        let s_22_0: bool = fn_state.gs_20290;
        // N s_22_1: branch s_22_0 b56 b23
        if s_22_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_23_0: read-var halfsize:i64
        let s_23_0: i64 = fn_state.halfsize;
        // D s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_2: call __id(s_23_1)
        let s_23_2: i128 = u__id(state, tracer, s_23_1);
        // D s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: const #8s : i
        let s_23_4: i128 = 8;
        // D s_23_5: cast zx s_23_3 -> i
        let s_23_5: i128 = (i128::try_from(s_23_3).unwrap());
        // D s_23_6: cmp-eq s_23_5 s_23_4
        let s_23_6: bool = ((s_23_5) == (s_23_4));
        // D s_23_7: write-var gs#20292 <= s_23_6
        fn_state.gs_20292 = s_23_6;
        // N s_23_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_24_0: read-var gs#20292:u8
        let s_24_0: bool = fn_state.gs_20292;
        // N s_24_1: assert s_24_0
        let s_24_1: () = assert!(s_24_0);
        // C s_24_2: const #() : ()
        let s_24_2: () = ();
        // S s_24_3: call HaveLRCPC3Ext(s_24_2)
        let s_24_3: bool = HaveLRCPC3Ext(state, tracer, s_24_2);
        // N s_24_4: branch s_24_3 b55 b25
        if s_24_3 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#20295 <= s_25_0
        fn_state.gs_20295 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_26_0: read-var gs#20295:u8
        let s_26_0: bool = fn_state.gs_20295;
        // N s_26_1: branch s_26_0 b54 b27
        if s_26_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#20296 <= s_27_0
        fn_state.gs_20296 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_28_0: read-var gs#20296:u8
        let s_28_0: bool = fn_state.gs_20296;
        // N s_28_1: branch s_28_0 b45 b29
        if s_28_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_29_0: const #1s : i64
        let s_29_0: i64 = 1;
        // D s_29_1: read-var address:u64
        let s_29_1: u64 = fn_state.address;
        // D s_29_2: read-var accdesc:struct
        let s_29_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_29_3: read-var aligned:u8
        let s_29_3: bool = fn_state.aligned;
        // D s_29_4: call AArch64_MemSingle_read(s_29_1, s_29_0, s_29_2, s_29_3)
        let s_29_4: Bits = AArch64_MemSingle_read(
            state,
            tracer,
            s_29_1,
            s_29_0,
            s_29_2,
            s_29_3,
        );
        // D s_29_5: write-var gs#447032 <= s_29_4
        fn_state.gs_447032 = s_29_4;
        // N s_29_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_30_0: read-var gs#447032:bv
        let s_30_0: Bits = fn_state.gs_447032;
        // D s_30_1: cast reint s_30_0 -> u8
        let s_30_1: u8 = (s_30_0.value() as u8);
        // C s_30_2: const #0s : i
        let s_30_2: i128 = 0;
        // D s_30_3: cast zx s_30_1 -> bv
        let s_30_3: Bits = Bits::new(s_30_1 as u128, 8u16);
        // D s_30_4: read-var value_name:bv
        let s_30_4: Bits = fn_state.value_name;
        // C s_30_5: const #7s : i
        let s_30_5: i128 = 7;
        // C s_30_6: const #1u : u64
        let s_30_6: u64 = 1;
        // C s_30_7: cast zx s_30_6 -> bv
        let s_30_7: Bits = Bits::new(s_30_6 as u128, 64u16);
        // C s_30_8: lsl s_30_7 s_30_5
        let s_30_8: Bits = s_30_7 << s_30_5;
        // C s_30_9: sub s_30_8 s_30_7
        let s_30_9: Bits = ((s_30_8) - (s_30_7));
        // D s_30_10: and s_30_3 s_30_9
        let s_30_10: Bits = ((s_30_3) & (s_30_9));
        // D s_30_11: lsl s_30_10 s_30_2
        let s_30_11: Bits = s_30_10 << s_30_2;
        // C s_30_12: lsl s_30_9 s_30_2
        let s_30_12: Bits = s_30_9 << s_30_2;
        // C s_30_13: cmpl s_30_12
        let s_30_13: Bits = !s_30_12;
        // D s_30_14: and s_30_4 s_30_13
        let s_30_14: Bits = ((s_30_4) & (s_30_13));
        // D s_30_15: or s_30_14 s_30_11
        let s_30_15: Bits = ((s_30_14) | (s_30_11));
        // D s_30_16: write-var value_name <= s_30_15
        fn_state.value_name = s_30_15;
        // C s_30_17: const #6u : u32
        let s_30_17: u32 = 6;
        // S s_30_18: call ConstrainUnpredictable(s_30_17)
        let s_30_18: u32 = ConstrainUnpredictable(state, tracer, s_30_17);
        // D s_30_19: write-var cshadow#328 <= s_30_18
        fn_state.cshadow_328 = s_30_18;
        // D s_30_20: read-var cshadow#328:u32
        let s_30_20: u32 = fn_state.cshadow_328;
        // C s_30_21: const #12u : u32
        let s_30_21: u32 = 12;
        // D s_30_22: cmp-eq s_30_20 s_30_21
        let s_30_22: bool = ((s_30_20) == (s_30_21));
        // N s_30_23: branch s_30_22 b44 b31
        if s_30_22 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_31_0: read-var cshadow#328:u32
        let s_31_0: u32 = fn_state.cshadow_328;
        // C s_31_1: const #0u : u32
        let s_31_1: u32 = 0;
        // D s_31_2: cmp-eq s_31_0 s_31_1
        let s_31_2: bool = ((s_31_0) == (s_31_1));
        // D s_31_3: write-var gs#20301 <= s_31_2
        fn_state.gs_20301 = s_31_2;
        // N s_31_4: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_32_0: read-var gs#20301:u8
        let s_32_0: bool = fn_state.gs_20301;
        // N s_32_1: assert s_32_0
        let s_32_1: () = assert!(s_32_0);
        // D s_32_2: read-var cshadow#328:u32
        let s_32_2: u32 = fn_state.cshadow_328;
        // C s_32_3: const #0u : u32
        let s_32_3: u32 = 0;
        // D s_32_4: cmp-eq s_32_2 s_32_3
        let s_32_4: bool = ((s_32_2) == (s_32_3));
        // N s_32_5: branch s_32_4 b43 b33
        if s_32_4 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_34_0: const #1s : i64
        let s_34_0: i64 = 1;
        // C s_34_1: const #1s : i
        let s_34_1: i128 = 1;
        // D s_34_2: read-var size:i64
        let s_34_2: i64 = fn_state.size;
        // D s_34_3: cast zx s_34_2 -> i
        let s_34_3: i128 = (i128::try_from(s_34_2).unwrap());
        // D s_34_4: sub s_34_3 s_34_1
        let s_34_4: i128 = ((s_34_3) - (s_34_1));
        // D s_34_5: cast reint s_34_4 -> i64
        let s_34_5: i64 = (s_34_4 as i64);
        // D s_34_6: write-var gs#20307 <= s_34_5
        fn_state.gs_20307 = s_34_5;
        // D s_34_7: write-var u#927 <= s_34_0
        fn_state.u_927 = s_34_0;
        // N s_34_8: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_35_0: read-var u#927:i64
        let s_35_0: i64 = fn_state.u_927;
        // D s_35_1: read-var gs#20307:i64
        let s_35_1: i64 = fn_state.gs_20307;
        // D s_35_2: cmp-gt s_35_0 s_35_1
        let s_35_2: bool = ((s_35_0) > (s_35_1));
        // N s_35_3: branch s_35_2 b38 b36
        if s_35_2 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_36_0: const #8s : i
        let s_36_0: i128 = 8;
        // D s_36_1: read-var u#927:i64
        let s_36_1: i64 = fn_state.u_927;
        // D s_36_2: cast zx s_36_1 -> i
        let s_36_2: i128 = (i128::try_from(s_36_1).unwrap());
        // D s_36_3: mul s_36_0 s_36_2
        let s_36_3: i128 = ((s_36_0) * (s_36_2));
        // D s_36_4: cast reint s_36_3 -> i64
        let s_36_4: i64 = (s_36_3 as i64);
        // C s_36_5: const #7s : i
        let s_36_5: i128 = 7;
        // D s_36_6: cast zx s_36_4 -> i
        let s_36_6: i128 = (i128::try_from(s_36_4).unwrap());
        // D s_36_7: add s_36_6 s_36_5
        let s_36_7: i128 = (s_36_6 + s_36_5);
        // D s_36_8: cast reint s_36_7 -> i64
        let s_36_8: i64 = (s_36_7 as i64);
        // D s_36_9: write-var ga#15764 <= s_36_8
        fn_state.ga_15764 = s_36_8;
        // C s_36_10: const #8s : i
        let s_36_10: i128 = 8;
        // D s_36_11: read-var u#927:i64
        let s_36_11: i64 = fn_state.u_927;
        // D s_36_12: cast zx s_36_11 -> i
        let s_36_12: i128 = (i128::try_from(s_36_11).unwrap());
        // D s_36_13: mul s_36_10 s_36_12
        let s_36_13: i128 = ((s_36_10) * (s_36_12));
        // D s_36_14: cast reint s_36_13 -> i64
        let s_36_14: i64 = (s_36_13 as i64);
        // D s_36_15: write-var ga#15765 <= s_36_14
        fn_state.ga_15765 = s_36_14;
        // D s_36_16: read-var address:u64
        let s_36_16: u64 = fn_state.address;
        // D s_36_17: cast zx s_36_16 -> bv
        let s_36_17: Bits = Bits::new(s_36_16 as u128, 64u16);
        // D s_36_18: read-var u#927:i64
        let s_36_18: i64 = fn_state.u_927;
        // D s_36_19: cast zx s_36_18 -> i
        let s_36_19: i128 = (i128::try_from(s_36_18).unwrap());
        // D s_36_20: cast cvt s_36_19 -> bv
        let s_36_20: Bits = Bits::new(s_36_19 as u128, 128);
        // D s_36_21: add s_36_17 s_36_20
        let s_36_21: Bits = (s_36_17 + s_36_20);
        // D s_36_22: cast reint s_36_21 -> u64
        let s_36_22: u64 = (s_36_21.value() as u64);
        // C s_36_23: const #1s : i64
        let s_36_23: i64 = 1;
        // D s_36_24: read-var accdesc:struct
        let s_36_24: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_36_25: read-var aligned:u8
        let s_36_25: bool = fn_state.aligned;
        // D s_36_26: call AArch64_MemSingle_read(s_36_22, s_36_23, s_36_24, s_36_25)
        let s_36_26: Bits = AArch64_MemSingle_read(
            state,
            tracer,
            s_36_22,
            s_36_23,
            s_36_24,
            s_36_25,
        );
        // D s_36_27: write-var gs#447043 <= s_36_26
        fn_state.gs_447043 = s_36_26;
        // N s_36_28: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_37_0: read-var gs#447043:bv
        let s_37_0: Bits = fn_state.gs_447043;
        // D s_37_1: cast reint s_37_0 -> u8
        let s_37_1: u8 = (s_37_0.value() as u8);
        // D s_37_2: read-var ga#15764:i64
        let s_37_2: i64 = fn_state.ga_15764;
        // D s_37_3: cast zx s_37_2 -> i
        let s_37_3: i128 = (i128::try_from(s_37_2).unwrap());
        // D s_37_4: read-var ga#15765:i64
        let s_37_4: i64 = fn_state.ga_15765;
        // D s_37_5: cast zx s_37_4 -> i
        let s_37_5: i128 = (i128::try_from(s_37_4).unwrap());
        // D s_37_6: cast zx s_37_1 -> bv
        let s_37_6: Bits = Bits::new(s_37_1 as u128, 8u16);
        // D s_37_7: read-var value_name:bv
        let s_37_7: Bits = fn_state.value_name;
        // D s_37_8: sub s_37_3 s_37_5
        let s_37_8: i128 = ((s_37_3) - (s_37_5));
        // C s_37_9: const #1u : u64
        let s_37_9: u64 = 1;
        // C s_37_10: cast zx s_37_9 -> bv
        let s_37_10: Bits = Bits::new(s_37_9 as u128, 64u16);
        // D s_37_11: lsl s_37_10 s_37_8
        let s_37_11: Bits = s_37_10 << s_37_8;
        // D s_37_12: sub s_37_11 s_37_10
        let s_37_12: Bits = ((s_37_11) - (s_37_10));
        // D s_37_13: and s_37_6 s_37_12
        let s_37_13: Bits = ((s_37_6) & (s_37_12));
        // D s_37_14: lsl s_37_13 s_37_5
        let s_37_14: Bits = s_37_13 << s_37_5;
        // D s_37_15: lsl s_37_12 s_37_5
        let s_37_15: Bits = s_37_12 << s_37_5;
        // D s_37_16: cmpl s_37_15
        let s_37_16: Bits = !s_37_15;
        // D s_37_17: and s_37_7 s_37_16
        let s_37_17: Bits = ((s_37_7) & (s_37_16));
        // D s_37_18: or s_37_17 s_37_14
        let s_37_18: Bits = ((s_37_17) | (s_37_14));
        // D s_37_19: write-var value_name <= s_37_18
        fn_state.value_name = s_37_18;
        // D s_37_20: read-var u#927:i64
        let s_37_20: i64 = fn_state.u_927;
        // C s_37_21: const #1s : i64
        let s_37_21: i64 = 1;
        // D s_37_22: add s_37_20 s_37_21
        let s_37_22: i64 = (s_37_20 + s_37_21);
        // D s_37_23: write-var u#927 <= s_37_22
        fn_state.u_927 = s_37_22;
        // N s_37_24: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_38_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_39_0: read-var accdesc.1:struct
        let s_39_0: u32 = fn_state.accdesc._1;
        // D s_39_1: call BigEndian(s_39_0)
        let s_39_1: bool = BigEndian(state, tracer, s_39_0);
        // N s_39_2: branch s_39_1 b42 b40
        if s_39_1 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_40_0: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_41_0: read-var value_name:bv
        let s_41_0: Bits = fn_state.value_name;
        // N s_41_1: return s_41_0
        return s_41_0;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_42_0: read-var value_name:bv
        let s_42_0: Bits = fn_state.value_name;
        // D s_42_1: call reverse_endianness(s_42_0)
        let s_42_1: Bits = reverse_endianness(state, tracer, s_42_0);
        // D s_42_2: write-var value_name <= s_42_1
        fn_state.value_name = s_42_1;
        // N s_42_3: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var aligned <= s_43_0
        fn_state.aligned = s_43_0;
        // N s_43_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#20301 <= s_44_0
        fn_state.gs_20301 = s_44_0;
        // N s_44_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_45_0: const #0s : i64
        let s_45_0: i64 = 0;
        // C s_45_1: const #1s : i
        let s_45_1: i128 = 1;
        // D s_45_2: read-var halfsize:i64
        let s_45_2: i64 = fn_state.halfsize;
        // D s_45_3: cast zx s_45_2 -> i
        let s_45_3: i128 = (i128::try_from(s_45_2).unwrap());
        // D s_45_4: sub s_45_3 s_45_1
        let s_45_4: i128 = ((s_45_3) - (s_45_1));
        // D s_45_5: cast reint s_45_4 -> i64
        let s_45_5: i64 = (s_45_4 as i64);
        // D s_45_6: write-var gs#20318 <= s_45_5
        fn_state.gs_20318 = s_45_5;
        // D s_45_7: write-var i <= s_45_0
        fn_state.i = s_45_0;
        // N s_45_8: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_46_0: read-var i:i64
        let s_46_0: i64 = fn_state.i;
        // D s_46_1: read-var gs#20318:i64
        let s_46_1: i64 = fn_state.gs_20318;
        // D s_46_2: cmp-gt s_46_0 s_46_1
        let s_46_2: bool = ((s_46_0) > (s_46_1));
        // N s_46_3: branch s_46_2 b49 b47
        if s_46_2 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_47_0: const #8s : i
        let s_47_0: i128 = 8;
        // D s_47_1: read-var i:i64
        let s_47_1: i64 = fn_state.i;
        // D s_47_2: cast zx s_47_1 -> i
        let s_47_2: i128 = (i128::try_from(s_47_1).unwrap());
        // D s_47_3: mul s_47_0 s_47_2
        let s_47_3: i128 = ((s_47_0) * (s_47_2));
        // D s_47_4: cast reint s_47_3 -> i64
        let s_47_4: i64 = (s_47_3 as i64);
        // C s_47_5: const #7s : i
        let s_47_5: i128 = 7;
        // D s_47_6: cast zx s_47_4 -> i
        let s_47_6: i128 = (i128::try_from(s_47_4).unwrap());
        // D s_47_7: add s_47_6 s_47_5
        let s_47_7: i128 = (s_47_6 + s_47_5);
        // D s_47_8: cast reint s_47_7 -> i64
        let s_47_8: i64 = (s_47_7 as i64);
        // D s_47_9: write-var ga#15750 <= s_47_8
        fn_state.ga_15750 = s_47_8;
        // C s_47_10: const #8s : i
        let s_47_10: i128 = 8;
        // D s_47_11: read-var i:i64
        let s_47_11: i64 = fn_state.i;
        // D s_47_12: cast zx s_47_11 -> i
        let s_47_12: i128 = (i128::try_from(s_47_11).unwrap());
        // D s_47_13: mul s_47_10 s_47_12
        let s_47_13: i128 = ((s_47_10) * (s_47_12));
        // D s_47_14: cast reint s_47_13 -> i64
        let s_47_14: i64 = (s_47_13 as i64);
        // D s_47_15: write-var ga#15751 <= s_47_14
        fn_state.ga_15751 = s_47_14;
        // D s_47_16: read-var address:u64
        let s_47_16: u64 = fn_state.address;
        // D s_47_17: cast zx s_47_16 -> bv
        let s_47_17: Bits = Bits::new(s_47_16 as u128, 64u16);
        // D s_47_18: read-var halfsize:i64
        let s_47_18: i64 = fn_state.halfsize;
        // D s_47_19: cast zx s_47_18 -> i
        let s_47_19: i128 = (i128::try_from(s_47_18).unwrap());
        // D s_47_20: cast cvt s_47_19 -> bv
        let s_47_20: Bits = Bits::new(s_47_19 as u128, 128);
        // D s_47_21: add s_47_17 s_47_20
        let s_47_21: Bits = (s_47_17 + s_47_20);
        // D s_47_22: cast reint s_47_21 -> u64
        let s_47_22: u64 = (s_47_21.value() as u64);
        // D s_47_23: cast zx s_47_22 -> bv
        let s_47_23: Bits = Bits::new(s_47_22 as u128, 64u16);
        // D s_47_24: read-var i:i64
        let s_47_24: i64 = fn_state.i;
        // D s_47_25: cast zx s_47_24 -> i
        let s_47_25: i128 = (i128::try_from(s_47_24).unwrap());
        // D s_47_26: cast cvt s_47_25 -> bv
        let s_47_26: Bits = Bits::new(s_47_25 as u128, 128);
        // D s_47_27: add s_47_23 s_47_26
        let s_47_27: Bits = (s_47_23 + s_47_26);
        // D s_47_28: cast reint s_47_27 -> u64
        let s_47_28: u64 = (s_47_27.value() as u64);
        // C s_47_29: const #1s : i64
        let s_47_29: i64 = 1;
        // D s_47_30: read-var accdesc:struct
        let s_47_30: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_47_31: read-var aligned:u8
        let s_47_31: bool = fn_state.aligned;
        // D s_47_32: call AArch64_MemSingle_read(s_47_28, s_47_29, s_47_30, s_47_31)
        let s_47_32: Bits = AArch64_MemSingle_read(
            state,
            tracer,
            s_47_28,
            s_47_29,
            s_47_30,
            s_47_31,
        );
        // D s_47_33: write-var gs#447065 <= s_47_32
        fn_state.gs_447065 = s_47_32;
        // N s_47_34: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_48_0: read-var gs#447065:bv
        let s_48_0: Bits = fn_state.gs_447065;
        // D s_48_1: cast reint s_48_0 -> u8
        let s_48_1: u8 = (s_48_0.value() as u8);
        // D s_48_2: read-var ga#15750:i64
        let s_48_2: i64 = fn_state.ga_15750;
        // D s_48_3: cast zx s_48_2 -> i
        let s_48_3: i128 = (i128::try_from(s_48_2).unwrap());
        // D s_48_4: read-var ga#15751:i64
        let s_48_4: i64 = fn_state.ga_15751;
        // D s_48_5: cast zx s_48_4 -> i
        let s_48_5: i128 = (i128::try_from(s_48_4).unwrap());
        // D s_48_6: cast zx s_48_1 -> bv
        let s_48_6: Bits = Bits::new(s_48_1 as u128, 8u16);
        // D s_48_7: read-var u#924:bv
        let s_48_7: Bits = fn_state.u_924;
        // D s_48_8: sub s_48_3 s_48_5
        let s_48_8: i128 = ((s_48_3) - (s_48_5));
        // C s_48_9: const #1u : u64
        let s_48_9: u64 = 1;
        // C s_48_10: cast zx s_48_9 -> bv
        let s_48_10: Bits = Bits::new(s_48_9 as u128, 64u16);
        // D s_48_11: lsl s_48_10 s_48_8
        let s_48_11: Bits = s_48_10 << s_48_8;
        // D s_48_12: sub s_48_11 s_48_10
        let s_48_12: Bits = ((s_48_11) - (s_48_10));
        // D s_48_13: and s_48_6 s_48_12
        let s_48_13: Bits = ((s_48_6) & (s_48_12));
        // D s_48_14: lsl s_48_13 s_48_5
        let s_48_14: Bits = s_48_13 << s_48_5;
        // D s_48_15: lsl s_48_12 s_48_5
        let s_48_15: Bits = s_48_12 << s_48_5;
        // D s_48_16: cmpl s_48_15
        let s_48_16: Bits = !s_48_15;
        // D s_48_17: and s_48_7 s_48_16
        let s_48_17: Bits = ((s_48_7) & (s_48_16));
        // D s_48_18: or s_48_17 s_48_14
        let s_48_18: Bits = ((s_48_17) | (s_48_14));
        // D s_48_19: write-var u#924 <= s_48_18
        fn_state.u_924 = s_48_18;
        // D s_48_20: read-var i:i64
        let s_48_20: i64 = fn_state.i;
        // C s_48_21: const #1s : i64
        let s_48_21: i64 = 1;
        // D s_48_22: add s_48_20 s_48_21
        let s_48_22: i64 = (s_48_20 + s_48_21);
        // D s_48_23: write-var i <= s_48_22
        fn_state.i = s_48_22;
        // N s_48_24: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_49_0: const #0s : i64
        let s_49_0: i64 = 0;
        // C s_49_1: const #1s : i
        let s_49_1: i128 = 1;
        // D s_49_2: read-var halfsize:i64
        let s_49_2: i64 = fn_state.halfsize;
        // D s_49_3: cast zx s_49_2 -> i
        let s_49_3: i128 = (i128::try_from(s_49_2).unwrap());
        // D s_49_4: sub s_49_3 s_49_1
        let s_49_4: i128 = ((s_49_3) - (s_49_1));
        // D s_49_5: cast reint s_49_4 -> i64
        let s_49_5: i64 = (s_49_4 as i64);
        // D s_49_6: write-var gs#20329 <= s_49_5
        fn_state.gs_20329 = s_49_5;
        // D s_49_7: write-var u#926 <= s_49_0
        fn_state.u_926 = s_49_0;
        // N s_49_8: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_50_0: read-var u#926:i64
        let s_50_0: i64 = fn_state.u_926;
        // D s_50_1: read-var gs#20329:i64
        let s_50_1: i64 = fn_state.gs_20329;
        // D s_50_2: cmp-gt s_50_0 s_50_1
        let s_50_2: bool = ((s_50_0) > (s_50_1));
        // N s_50_3: branch s_50_2 b53 b51
        if s_50_2 {
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
        // C s_51_0: const #8s : i
        let s_51_0: i128 = 8;
        // D s_51_1: read-var u#926:i64
        let s_51_1: i64 = fn_state.u_926;
        // D s_51_2: cast zx s_51_1 -> i
        let s_51_2: i128 = (i128::try_from(s_51_1).unwrap());
        // D s_51_3: mul s_51_0 s_51_2
        let s_51_3: i128 = ((s_51_0) * (s_51_2));
        // D s_51_4: cast reint s_51_3 -> i64
        let s_51_4: i64 = (s_51_3 as i64);
        // C s_51_5: const #7s : i
        let s_51_5: i128 = 7;
        // D s_51_6: cast zx s_51_4 -> i
        let s_51_6: i128 = (i128::try_from(s_51_4).unwrap());
        // D s_51_7: add s_51_6 s_51_5
        let s_51_7: i128 = (s_51_6 + s_51_5);
        // D s_51_8: cast reint s_51_7 -> i64
        let s_51_8: i64 = (s_51_7 as i64);
        // D s_51_9: write-var ga#15755 <= s_51_8
        fn_state.ga_15755 = s_51_8;
        // C s_51_10: const #8s : i
        let s_51_10: i128 = 8;
        // D s_51_11: read-var u#926:i64
        let s_51_11: i64 = fn_state.u_926;
        // D s_51_12: cast zx s_51_11 -> i
        let s_51_12: i128 = (i128::try_from(s_51_11).unwrap());
        // D s_51_13: mul s_51_10 s_51_12
        let s_51_13: i128 = ((s_51_10) * (s_51_12));
        // D s_51_14: cast reint s_51_13 -> i64
        let s_51_14: i64 = (s_51_13 as i64);
        // D s_51_15: write-var ga#15756 <= s_51_14
        fn_state.ga_15756 = s_51_14;
        // D s_51_16: read-var address:u64
        let s_51_16: u64 = fn_state.address;
        // D s_51_17: cast zx s_51_16 -> bv
        let s_51_17: Bits = Bits::new(s_51_16 as u128, 64u16);
        // D s_51_18: read-var u#926:i64
        let s_51_18: i64 = fn_state.u_926;
        // D s_51_19: cast zx s_51_18 -> i
        let s_51_19: i128 = (i128::try_from(s_51_18).unwrap());
        // D s_51_20: cast cvt s_51_19 -> bv
        let s_51_20: Bits = Bits::new(s_51_19 as u128, 128);
        // D s_51_21: add s_51_17 s_51_20
        let s_51_21: Bits = (s_51_17 + s_51_20);
        // D s_51_22: cast reint s_51_21 -> u64
        let s_51_22: u64 = (s_51_21.value() as u64);
        // C s_51_23: const #1s : i64
        let s_51_23: i64 = 1;
        // D s_51_24: read-var accdesc:struct
        let s_51_24: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_51_25: read-var aligned:u8
        let s_51_25: bool = fn_state.aligned;
        // D s_51_26: call AArch64_MemSingle_read(s_51_22, s_51_23, s_51_24, s_51_25)
        let s_51_26: Bits = AArch64_MemSingle_read(
            state,
            tracer,
            s_51_22,
            s_51_23,
            s_51_24,
            s_51_25,
        );
        // D s_51_27: write-var gs#447078 <= s_51_26
        fn_state.gs_447078 = s_51_26;
        // N s_51_28: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_52_0: read-var gs#447078:bv
        let s_52_0: Bits = fn_state.gs_447078;
        // D s_52_1: cast reint s_52_0 -> u8
        let s_52_1: u8 = (s_52_0.value() as u8);
        // D s_52_2: read-var ga#15755:i64
        let s_52_2: i64 = fn_state.ga_15755;
        // D s_52_3: cast zx s_52_2 -> i
        let s_52_3: i128 = (i128::try_from(s_52_2).unwrap());
        // D s_52_4: read-var ga#15756:i64
        let s_52_4: i64 = fn_state.ga_15756;
        // D s_52_5: cast zx s_52_4 -> i
        let s_52_5: i128 = (i128::try_from(s_52_4).unwrap());
        // D s_52_6: cast zx s_52_1 -> bv
        let s_52_6: Bits = Bits::new(s_52_1 as u128, 8u16);
        // D s_52_7: read-var u#925:bv
        let s_52_7: Bits = fn_state.u_925;
        // D s_52_8: sub s_52_3 s_52_5
        let s_52_8: i128 = ((s_52_3) - (s_52_5));
        // C s_52_9: const #1u : u64
        let s_52_9: u64 = 1;
        // C s_52_10: cast zx s_52_9 -> bv
        let s_52_10: Bits = Bits::new(s_52_9 as u128, 64u16);
        // D s_52_11: lsl s_52_10 s_52_8
        let s_52_11: Bits = s_52_10 << s_52_8;
        // D s_52_12: sub s_52_11 s_52_10
        let s_52_12: Bits = ((s_52_11) - (s_52_10));
        // D s_52_13: and s_52_6 s_52_12
        let s_52_13: Bits = ((s_52_6) & (s_52_12));
        // D s_52_14: lsl s_52_13 s_52_5
        let s_52_14: Bits = s_52_13 << s_52_5;
        // D s_52_15: lsl s_52_12 s_52_5
        let s_52_15: Bits = s_52_12 << s_52_5;
        // D s_52_16: cmpl s_52_15
        let s_52_16: Bits = !s_52_15;
        // D s_52_17: and s_52_7 s_52_16
        let s_52_17: Bits = ((s_52_7) & (s_52_16));
        // D s_52_18: or s_52_17 s_52_14
        let s_52_18: Bits = ((s_52_17) | (s_52_14));
        // D s_52_19: write-var u#925 <= s_52_18
        fn_state.u_925 = s_52_18;
        // D s_52_20: read-var u#926:i64
        let s_52_20: i64 = fn_state.u_926;
        // C s_52_21: const #1s : i64
        let s_52_21: i64 = 1;
        // D s_52_22: add s_52_20 s_52_21
        let s_52_22: i64 = (s_52_20 + s_52_21);
        // D s_52_23: write-var u#926 <= s_52_22
        fn_state.u_926 = s_52_22;
        // N s_52_24: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_53_0: read-var u#924:bv
        let s_53_0: Bits = fn_state.u_924;
        // D s_53_1: read-var u#925:bv
        let s_53_1: Bits = fn_state.u_925;
        // D s_53_2: cast reint s_53_0 -> u128
        let s_53_2: u128 = (s_53_0.value() as u128);
        // D s_53_3: size-of s_53_0
        let s_53_3: u16 = s_53_0.length();
        // D s_53_4: cast reint s_53_1 -> u128
        let s_53_4: u128 = (s_53_1.value() as u128);
        // D s_53_5: size-of s_53_1
        let s_53_5: u16 = s_53_1.length();
        // D s_53_6: lsl s_53_2 s_53_5
        let s_53_6: u128 = s_53_2 << s_53_5;
        // D s_53_7: or s_53_6 s_53_4
        let s_53_7: u128 = ((s_53_6) | (s_53_4));
        // D s_53_8: add s_53_3 s_53_5
        let s_53_8: u16 = (s_53_3 + s_53_5);
        // D s_53_9: create-bits s_53_7 s_53_8
        let s_53_9: Bits = Bits::new(s_53_7, s_53_8);
        // D s_53_10: write-var value_name <= s_53_9
        fn_state.value_name = s_53_9;
        // N s_53_11: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_54_0: read-var highestAddressfirst:u8
        let s_54_0: bool = fn_state.highestAddressfirst;
        // D s_54_1: write-var gs#20296 <= s_54_0
        fn_state.gs_20296 = s_54_0;
        // N s_54_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_55_0: read-var ispair:u8
        let s_55_0: bool = fn_state.ispair;
        // D s_55_1: write-var gs#20295 <= s_55_0
        fn_state.gs_20295 = s_55_0;
        // N s_55_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#20292 <= s_56_0
        fn_state.gs_20292 = s_56_0;
        // N s_56_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#20290 <= s_57_0
        fn_state.gs_20290 = s_57_0;
        // N s_57_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#20288 <= s_58_0
        fn_state.gs_20288 = s_58_0;
        // N s_58_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_59_0: read-var address:u64
        let s_59_0: u64 = fn_state.address;
        // D s_59_1: read-var size:i64
        let s_59_1: i64 = fn_state.size;
        // D s_59_2: read-var accdesc:struct
        let s_59_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_59_3: read-var aligned:u8
        let s_59_3: bool = fn_state.aligned;
        // D s_59_4: read-var ispair:u8
        let s_59_4: bool = fn_state.ispair;
        // D s_59_5: call AArch64_MemSingle_read__1(s_59_0, s_59_1, s_59_2, s_59_3, s_59_4)
        let s_59_5: Bits = AArch64_MemSingle_read__1(
            state,
            tracer,
            s_59_0,
            s_59_1,
            s_59_2,
            s_59_3,
            s_59_4,
        );
        // D s_59_6: write-var value_name <= s_59_5
        fn_state.value_name = s_59_5;
        // N s_59_7: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_60_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_61_0: read-var halfsize:i64
        let s_61_0: i64 = fn_state.halfsize;
        // D s_61_1: cast zx s_61_0 -> i
        let s_61_1: i128 = (i128::try_from(s_61_0).unwrap());
        // D s_61_2: call __id(s_61_1)
        let s_61_2: i128 = u__id(state, tracer, s_61_1);
        // D s_61_3: cast reint s_61_2 -> i64
        let s_61_3: i64 = (s_61_2 as i64);
        // C s_61_4: const #1s : i
        let s_61_4: i128 = 1;
        // D s_61_5: cast zx s_61_3 -> i
        let s_61_5: i128 = (i128::try_from(s_61_3).unwrap());
        // D s_61_6: cmp-eq s_61_5 s_61_4
        let s_61_6: bool = ((s_61_5) == (s_61_4));
        // N s_61_7: branch s_61_6 b80 b62
        if s_61_6 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_62_0: read-var halfsize:i64
        let s_62_0: i64 = fn_state.halfsize;
        // D s_62_1: cast zx s_62_0 -> i
        let s_62_1: i128 = (i128::try_from(s_62_0).unwrap());
        // D s_62_2: call __id(s_62_1)
        let s_62_2: i128 = u__id(state, tracer, s_62_1);
        // D s_62_3: cast reint s_62_2 -> i64
        let s_62_3: i64 = (s_62_2 as i64);
        // C s_62_4: const #2s : i
        let s_62_4: i128 = 2;
        // D s_62_5: cast zx s_62_3 -> i
        let s_62_5: i128 = (i128::try_from(s_62_3).unwrap());
        // D s_62_6: cmp-eq s_62_5 s_62_4
        let s_62_6: bool = ((s_62_5) == (s_62_4));
        // D s_62_7: write-var gs#20341 <= s_62_6
        fn_state.gs_20341 = s_62_6;
        // N s_62_8: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_63_0: read-var gs#20341:u8
        let s_63_0: bool = fn_state.gs_20341;
        // N s_63_1: branch s_63_0 b79 b64
        if s_63_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_64_0: read-var halfsize:i64
        let s_64_0: i64 = fn_state.halfsize;
        // D s_64_1: cast zx s_64_0 -> i
        let s_64_1: i128 = (i128::try_from(s_64_0).unwrap());
        // D s_64_2: call __id(s_64_1)
        let s_64_2: i128 = u__id(state, tracer, s_64_1);
        // D s_64_3: cast reint s_64_2 -> i64
        let s_64_3: i64 = (s_64_2 as i64);
        // C s_64_4: const #4s : i
        let s_64_4: i128 = 4;
        // D s_64_5: cast zx s_64_3 -> i
        let s_64_5: i128 = (i128::try_from(s_64_3).unwrap());
        // D s_64_6: cmp-eq s_64_5 s_64_4
        let s_64_6: bool = ((s_64_5) == (s_64_4));
        // D s_64_7: write-var gs#20343 <= s_64_6
        fn_state.gs_20343 = s_64_6;
        // N s_64_8: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_65_0: read-var gs#20343:u8
        let s_65_0: bool = fn_state.gs_20343;
        // N s_65_1: branch s_65_0 b78 b66
        if s_65_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_66_0: read-var halfsize:i64
        let s_66_0: i64 = fn_state.halfsize;
        // D s_66_1: cast zx s_66_0 -> i
        let s_66_1: i128 = (i128::try_from(s_66_0).unwrap());
        // D s_66_2: call __id(s_66_1)
        let s_66_2: i128 = u__id(state, tracer, s_66_1);
        // D s_66_3: cast reint s_66_2 -> i64
        let s_66_3: i64 = (s_66_2 as i64);
        // C s_66_4: const #8s : i
        let s_66_4: i128 = 8;
        // D s_66_5: cast zx s_66_3 -> i
        let s_66_5: i128 = (i128::try_from(s_66_3).unwrap());
        // D s_66_6: cmp-eq s_66_5 s_66_4
        let s_66_6: bool = ((s_66_5) == (s_66_4));
        // D s_66_7: write-var gs#20345 <= s_66_6
        fn_state.gs_20345 = s_66_6;
        // N s_66_8: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_67_0: read-var gs#20345:u8
        let s_67_0: bool = fn_state.gs_20345;
        // N s_67_1: assert s_67_0
        let s_67_1: () = assert!(s_67_0);
        // C s_67_2: const #() : ()
        let s_67_2: () = ();
        // S s_67_3: call HaveLRCPC3Ext(s_67_2)
        let s_67_3: bool = HaveLRCPC3Ext(state, tracer, s_67_2);
        // N s_67_4: branch s_67_3 b77 b68
        if s_67_3 {
            return block_77(state, tracer, fn_state);
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
        // D s_68_1: write-var gs#20347 <= s_68_0
        fn_state.gs_20347 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_69_0: read-var gs#20347:u8
        let s_69_0: bool = fn_state.gs_20347;
        // N s_69_1: branch s_69_0 b74 b70
        if s_69_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_70_0: read-var address:u64
        let s_70_0: u64 = fn_state.address;
        // D s_70_1: read-var halfsize:i64
        let s_70_1: i64 = fn_state.halfsize;
        // D s_70_2: read-var accdesc:struct
        let s_70_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_70_3: read-var aligned:u8
        let s_70_3: bool = fn_state.aligned;
        // D s_70_4: call AArch64_MemSingle_read(s_70_0, s_70_1, s_70_2, s_70_3)
        let s_70_4: Bits = AArch64_MemSingle_read(
            state,
            tracer,
            s_70_0,
            s_70_1,
            s_70_2,
            s_70_3,
        );
        // D s_70_5: write-var u#923 <= s_70_4
        fn_state.u_923 = s_70_4;
        // N s_70_6: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_71_0: read-var address:u64
        let s_71_0: u64 = fn_state.address;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 64u16);
        // D s_71_2: read-var halfsize:i64
        let s_71_2: i64 = fn_state.halfsize;
        // D s_71_3: cast zx s_71_2 -> i
        let s_71_3: i128 = (i128::try_from(s_71_2).unwrap());
        // D s_71_4: cast cvt s_71_3 -> bv
        let s_71_4: Bits = Bits::new(s_71_3 as u128, 128);
        // D s_71_5: add s_71_1 s_71_4
        let s_71_5: Bits = (s_71_1 + s_71_4);
        // D s_71_6: cast reint s_71_5 -> u64
        let s_71_6: u64 = (s_71_5.value() as u64);
        // D s_71_7: read-var halfsize:i64
        let s_71_7: i64 = fn_state.halfsize;
        // D s_71_8: read-var accdesc:struct
        let s_71_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_71_9: read-var aligned:u8
        let s_71_9: bool = fn_state.aligned;
        // D s_71_10: call AArch64_MemSingle_read(s_71_6, s_71_7, s_71_8, s_71_9)
        let s_71_10: Bits = AArch64_MemSingle_read(
            state,
            tracer,
            s_71_6,
            s_71_7,
            s_71_8,
            s_71_9,
        );
        // D s_71_11: write-var u#922 <= s_71_10
        fn_state.u_922 = s_71_10;
        // N s_71_12: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_72_0: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_73_0: read-var u#922:bv
        let s_73_0: Bits = fn_state.u_922;
        // D s_73_1: read-var u#923:bv
        let s_73_1: Bits = fn_state.u_923;
        // D s_73_2: cast reint s_73_0 -> u128
        let s_73_2: u128 = (s_73_0.value() as u128);
        // D s_73_3: size-of s_73_0
        let s_73_3: u16 = s_73_0.length();
        // D s_73_4: cast reint s_73_1 -> u128
        let s_73_4: u128 = (s_73_1.value() as u128);
        // D s_73_5: size-of s_73_1
        let s_73_5: u16 = s_73_1.length();
        // D s_73_6: lsl s_73_2 s_73_5
        let s_73_6: u128 = s_73_2 << s_73_5;
        // D s_73_7: or s_73_6 s_73_4
        let s_73_7: u128 = ((s_73_6) | (s_73_4));
        // D s_73_8: add s_73_3 s_73_5
        let s_73_8: u16 = (s_73_3 + s_73_5);
        // D s_73_9: create-bits s_73_7 s_73_8
        let s_73_9: Bits = Bits::new(s_73_7, s_73_8);
        // D s_73_10: write-var value_name <= s_73_9
        fn_state.value_name = s_73_9;
        // N s_73_11: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_74_0: read-var address:u64
        let s_74_0: u64 = fn_state.address;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 64u16);
        // D s_74_2: read-var halfsize:i64
        let s_74_2: i64 = fn_state.halfsize;
        // D s_74_3: cast zx s_74_2 -> i
        let s_74_3: i128 = (i128::try_from(s_74_2).unwrap());
        // D s_74_4: cast cvt s_74_3 -> bv
        let s_74_4: Bits = Bits::new(s_74_3 as u128, 128);
        // D s_74_5: add s_74_1 s_74_4
        let s_74_5: Bits = (s_74_1 + s_74_4);
        // D s_74_6: cast reint s_74_5 -> u64
        let s_74_6: u64 = (s_74_5.value() as u64);
        // D s_74_7: read-var halfsize:i64
        let s_74_7: i64 = fn_state.halfsize;
        // D s_74_8: read-var accdesc:struct
        let s_74_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_74_9: read-var aligned:u8
        let s_74_9: bool = fn_state.aligned;
        // D s_74_10: call AArch64_MemSingle_read(s_74_6, s_74_7, s_74_8, s_74_9)
        let s_74_10: Bits = AArch64_MemSingle_read(
            state,
            tracer,
            s_74_6,
            s_74_7,
            s_74_8,
            s_74_9,
        );
        // D s_74_11: write-var u#922 <= s_74_10
        fn_state.u_922 = s_74_10;
        // N s_74_12: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_75_0: read-var address:u64
        let s_75_0: u64 = fn_state.address;
        // D s_75_1: read-var halfsize:i64
        let s_75_1: i64 = fn_state.halfsize;
        // D s_75_2: read-var accdesc:struct
        let s_75_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_75_3: read-var aligned:u8
        let s_75_3: bool = fn_state.aligned;
        // D s_75_4: call AArch64_MemSingle_read(s_75_0, s_75_1, s_75_2, s_75_3)
        let s_75_4: Bits = AArch64_MemSingle_read(
            state,
            tracer,
            s_75_0,
            s_75_1,
            s_75_2,
            s_75_3,
        );
        // D s_75_5: write-var u#923 <= s_75_4
        fn_state.u_923 = s_75_4;
        // N s_75_6: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_76_0: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_77_0: read-var highestAddressfirst:u8
        let s_77_0: bool = fn_state.highestAddressfirst;
        // D s_77_1: write-var gs#20347 <= s_77_0
        fn_state.gs_20347 = s_77_0;
        // N s_77_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_78_0: const #1u : u8
        let s_78_0: bool = true;
        // D s_78_1: write-var gs#20345 <= s_78_0
        fn_state.gs_20345 = s_78_0;
        // N s_78_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // D s_79_1: write-var gs#20343 <= s_79_0
        fn_state.gs_20343 = s_79_0;
        // N s_79_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_80_0: const #1u : u8
        let s_80_0: bool = true;
        // D s_80_1: write-var gs#20341 <= s_80_0
        fn_state.gs_20341 = s_80_0;
        // N s_80_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_81_0: read-var aligned:u8
        let s_81_0: bool = fn_state.aligned;
        // D s_81_1: write-var gs#20284 <= s_81_0
        fn_state.gs_20284 = s_81_0;
        // N s_81_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_82_0: read-var address:u64
        let s_82_0: u64 = fn_state.address;
        // D s_82_1: read-var size:i64
        let s_82_1: i64 = fn_state.size;
        // D s_82_2: read-var accdesc:struct
        let s_82_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_82_3: read-var aligned:u8
        let s_82_3: bool = fn_state.aligned;
        // D s_82_4: read-var ispair:u8
        let s_82_4: bool = fn_state.ispair;
        // D s_82_5: call AArch64_MemSingle_read__1(s_82_0, s_82_1, s_82_2, s_82_3, s_82_4)
        let s_82_5: Bits = AArch64_MemSingle_read__1(
            state,
            tracer,
            s_82_0,
            s_82_1,
            s_82_2,
            s_82_3,
            s_82_4,
        );
        // D s_82_6: write-var value_name <= s_82_5
        fn_state.value_name = s_82_5;
        // N s_82_7: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_83_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_84_0: const #16s : i
        let s_84_0: i128 = 16;
        // D s_84_1: read-var size:i64
        let s_84_1: i64 = fn_state.size;
        // D s_84_2: cast zx s_84_1 -> i
        let s_84_2: i128 = (i128::try_from(s_84_1).unwrap());
        // D s_84_3: read-var address:u64
        let s_84_3: u64 = fn_state.address;
        // D s_84_4: call AllInAlignedQuantity(s_84_3, s_84_2, s_84_0)
        let s_84_4: bool = AllInAlignedQuantity(state, tracer, s_84_3, s_84_2, s_84_0);
        // D s_84_5: write-var gs#20283 <= s_84_4
        fn_state.gs_20283 = s_84_4;
        // N s_84_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_85_0: read-var address:u64
        let s_85_0: u64 = fn_state.address;
        // D s_85_1: read-var halfsize:i64
        let s_85_1: i64 = fn_state.halfsize;
        // D s_85_2: read-var accdesc:struct
        let s_85_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_85_3: read-var aligned:u8
        let s_85_3: bool = fn_state.aligned;
        // D s_85_4: read-var ispair:u8
        let s_85_4: bool = fn_state.ispair;
        // D s_85_5: call AArch64_MemSingle_read__1(s_85_0, s_85_1, s_85_2, s_85_3, s_85_4)
        let s_85_5: Bits = AArch64_MemSingle_read__1(
            state,
            tracer,
            s_85_0,
            s_85_1,
            s_85_2,
            s_85_3,
            s_85_4,
        );
        // D s_85_6: write-var lowhalf <= s_85_5
        fn_state.lowhalf = s_85_5;
        // N s_85_7: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_86_0: read-var address:u64
        let s_86_0: u64 = fn_state.address;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 64u16);
        // D s_86_2: read-var halfsize:i64
        let s_86_2: i64 = fn_state.halfsize;
        // D s_86_3: cast zx s_86_2 -> i
        let s_86_3: i128 = (i128::try_from(s_86_2).unwrap());
        // D s_86_4: cast cvt s_86_3 -> bv
        let s_86_4: Bits = Bits::new(s_86_3 as u128, 128);
        // D s_86_5: add s_86_1 s_86_4
        let s_86_5: Bits = (s_86_1 + s_86_4);
        // D s_86_6: cast reint s_86_5 -> u64
        let s_86_6: u64 = (s_86_5.value() as u64);
        // D s_86_7: read-var halfsize:i64
        let s_86_7: i64 = fn_state.halfsize;
        // D s_86_8: read-var accdesc:struct
        let s_86_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_86_9: read-var aligned:u8
        let s_86_9: bool = fn_state.aligned;
        // D s_86_10: read-var ispair:u8
        let s_86_10: bool = fn_state.ispair;
        // D s_86_11: call AArch64_MemSingle_read__1(s_86_6, s_86_7, s_86_8, s_86_9, s_86_10)
        let s_86_11: Bits = AArch64_MemSingle_read__1(
            state,
            tracer,
            s_86_6,
            s_86_7,
            s_86_8,
            s_86_9,
            s_86_10,
        );
        // D s_86_12: write-var highhalf <= s_86_11
        fn_state.highhalf = s_86_11;
        // N s_86_13: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_87_0: read-var highhalf:bv
        let s_87_0: Bits = fn_state.highhalf;
        // D s_87_1: read-var lowhalf:bv
        let s_87_1: Bits = fn_state.lowhalf;
        // D s_87_2: cast reint s_87_0 -> u128
        let s_87_2: u128 = (s_87_0.value() as u128);
        // D s_87_3: size-of s_87_0
        let s_87_3: u16 = s_87_0.length();
        // D s_87_4: cast reint s_87_1 -> u128
        let s_87_4: u128 = (s_87_1.value() as u128);
        // D s_87_5: size-of s_87_1
        let s_87_5: u16 = s_87_1.length();
        // D s_87_6: lsl s_87_2 s_87_5
        let s_87_6: u128 = s_87_2 << s_87_5;
        // D s_87_7: or s_87_6 s_87_4
        let s_87_7: u128 = ((s_87_6) | (s_87_4));
        // D s_87_8: add s_87_3 s_87_5
        let s_87_8: u16 = (s_87_3 + s_87_5);
        // D s_87_9: create-bits s_87_7 s_87_8
        let s_87_9: Bits = Bits::new(s_87_7, s_87_8);
        // D s_87_10: write-var value_name <= s_87_9
        fn_state.value_name = s_87_9;
        // N s_87_11: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_88_0: const #8s : i
        let s_88_0: i128 = 8;
        // D s_88_1: read-var address:u64
        let s_88_1: u64 = fn_state.address;
        // D s_88_2: cast zx s_88_1 -> bv
        let s_88_2: Bits = Bits::new(s_88_1 as u128, 64u16);
        // D s_88_3: call IsAligned__1(s_88_2, s_88_0)
        let s_88_3: bool = IsAligned__1(state, tracer, s_88_2, s_88_0);
        // D s_88_4: write-var gs#20279 <= s_88_3
        fn_state.gs_20279 = s_88_3;
        // N s_88_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_89_0: const #16s : i
        let s_89_0: i128 = 16;
        // D s_89_1: read-var size:i64
        let s_89_1: i64 = fn_state.size;
        // D s_89_2: cast zx s_89_1 -> i
        let s_89_2: i128 = (i128::try_from(s_89_1).unwrap());
        // D s_89_3: cmp-eq s_89_2 s_89_0
        let s_89_3: bool = ((s_89_2) == (s_89_0));
        // D s_89_4: write-var gs#20277 <= s_89_3
        fn_state.gs_20277 = s_89_3;
        // N s_89_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_90_0: read-var accdesc:struct
        let s_90_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_90_1: call AlignmentFault(s_90_0)
        let s_90_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_90_0);
        // D s_90_2: read-var address:u64
        let s_90_2: u64 = fn_state.address;
        // D s_90_3: call AArch64_Abort(s_90_2, s_90_1)
        let s_90_3: () = AArch64_Abort(state, tracer, s_90_2, s_90_1);
        // N s_90_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_91_0: read-var size:i64
        let s_91_0: i64 = fn_state.size;
        // D s_91_1: cast zx s_91_0 -> i
        let s_91_1: i128 = (i128::try_from(s_91_0).unwrap());
        // D s_91_2: read-var accdesc:struct
        let s_91_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_91_3: read-var address:u64
        let s_91_3: u64 = fn_state.address;
        // D s_91_4: call AArch64_UnalignedAccessFaults(s_91_2, s_91_3, s_91_1)
        let s_91_4: bool = AArch64_UnalignedAccessFaults(
            state,
            tracer,
            s_91_2,
            s_91_3,
            s_91_1,
        );
        // D s_91_5: write-var gs#20275 <= s_91_4
        fn_state.gs_20275 = s_91_4;
        // N s_91_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_92_0: read-var halfsize:i64
        let s_92_0: i64 = fn_state.halfsize;
        // D s_92_1: write-var ga#15711 <= s_92_0
        fn_state.ga_15711 = s_92_0;
        // N s_92_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
