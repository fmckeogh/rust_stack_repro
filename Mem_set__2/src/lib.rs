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
use AArch64_MemSingle_set__1::*;
use u__id::*;
use HaveLSE2Ext::*;
use AllInAlignedQuantity::*;
use AArch64_Abort::*;
use BigEndian::*;
use IsAligned__1::*;
use HaveLRCPC3Ext::*;
use ConstrainUnpredictable::*;
use AArch64_MemSingle_set::*;
use reverse_endianness::*;
use AlignmentFault::*;
use common::*;
pub fn Mem_set__2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    size: i128,
    accdesc: ProductType9878976b5bcce9c9,
    ispair: bool,
    highestAddressfirst: bool,
    value_in_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_20468: bool,
        gs_20374: bool,
        u_933: i64,
        gs_20477: bool,
        gs_20452: bool,
        gs_20364: bool,
        value_name: Bits,
        gs_20417: i64,
        gs_20357: bool,
        gs_20506: bool,
        gs_20359: bool,
        halfsize: i128,
        gs_20508: bool,
        u_929: Bits,
        gs_20505: bool,
        gs_20507: bool,
        gs_20479: bool,
        gs_20361: bool,
        gs_20470: bool,
        u_930: Bits,
        gs_20369: bool,
        gs_20432: bool,
        gs_20380: i64,
        i: i64,
        cshadow_329: u32,
        u_931: Bits,
        gs_20404: i64,
        gs_20430: bool,
        gs_20461: bool,
        gs_20481: bool,
        gs_20457: bool,
        gs_20455: bool,
        gs_20466: bool,
        gs_20368: bool,
        u_932: i64,
        highhalf: Bits,
        gs_20426: bool,
        ga_15770: i128,
        u_928: Bits,
        gs_20472: bool,
        gs_20365: bool,
        gs_20459: bool,
        gs_20483: bool,
        gs_20428: bool,
        aligned: bool,
        address: u64,
        size: i128,
        accdesc: ProductType9878976b5bcce9c9,
        ispair: bool,
        highestAddressfirst: bool,
        value_in_name: Bits,
    }
    let fn_state = FunctionState {
        address,
        size,
        accdesc,
        ispair,
        highestAddressfirst,
        value_in_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #2s : i
        let s_0_0: i128 = 2;
        // D s_0_1: read-var size:i
        let s_0_1: i128 = fn_state.size;
        // D s_0_2: div s_0_1 s_0_0
        let s_0_2: i128 = ((s_0_1) / (s_0_0));
        // D s_0_3: write-var halfsize <= s_0_2
        fn_state.halfsize = s_0_2;
        // D s_0_4: read-var value_in_name:bv
        let s_0_4: Bits = fn_state.value_in_name;
        // D s_0_5: write-var value_name <= s_0_4
        fn_state.value_name = s_0_4;
        // D s_0_6: read-var ispair:u8
        let s_0_6: bool = fn_state.ispair;
        // N s_0_7: branch s_0_6 b127 b1
        if s_0_6 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:i
        let s_1_0: i128 = fn_state.size;
        // D s_1_1: write-var ga#15770 <= s_1_0
        fn_state.ga_15770 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#15770:i
        let s_2_0: i128 = fn_state.ga_15770;
        // D s_2_1: read-var address:u64
        let s_2_1: u64 = fn_state.address;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 64u16);
        // D s_2_3: call IsAligned__1(s_2_2, s_2_0)
        let s_2_3: bool = IsAligned__1(state, tracer, s_2_2, s_2_0);
        // D s_2_4: write-var aligned <= s_2_3
        fn_state.aligned = s_2_3;
        // D s_2_5: read-var aligned:u8
        let s_2_5: bool = fn_state.aligned;
        // D s_2_6: not s_2_5
        let s_2_6: bool = !s_2_5;
        // N s_2_7: branch s_2_6 b126 b3
        if s_2_6 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#20357 <= s_3_0
        fn_state.gs_20357 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#20357:u8
        let s_4_0: bool = fn_state.gs_20357;
        // N s_4_1: branch s_4_0 b125 b5
        if s_4_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var accdesc.1:struct
        let s_6_0: u32 = fn_state.accdesc._1;
        // D s_6_1: call BigEndian(s_6_0)
        let s_6_1: bool = BigEndian(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b112 b7
        if s_6_1 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var accdesc.1:struct
        let s_8_0: u32 = fn_state.accdesc._1;
        // C s_8_1: const #2u : u32
        let s_8_1: u32 = 2;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b111 b9
        if s_8_2 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#20359 <= s_9_0
        fn_state.gs_20359 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#20359:u8
        let s_10_0: bool = fn_state.gs_20359;
        // N s_10_1: branch s_10_0 b110 b11
        if s_10_0 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#20361 <= s_11_0
        fn_state.gs_20361 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#20361:u8
        let s_12_0: bool = fn_state.gs_20361;
        // N s_12_1: branch s_12_0 b108 b13
        if s_12_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call HaveLSE2Ext(s_13_0)
        let s_13_1: bool = HaveLSE2Ext(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b107 b14
        if s_13_1 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#20364 <= s_14_0
        fn_state.gs_20364 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#20364:u8
        let s_15_0: bool = fn_state.gs_20364;
        // N s_15_1: branch s_15_0 b94 b16
        if s_15_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var ispair:u8
        let s_16_0: bool = fn_state.ispair;
        // N s_16_1: branch s_16_0 b93 b17
        if s_16_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#20365 <= s_17_0
        fn_state.gs_20365 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#20365:u8
        let s_18_0: bool = fn_state.gs_20365;
        // N s_18_1: branch s_18_0 b61 b19
        if s_18_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var aligned:u8
        let s_19_0: bool = fn_state.aligned;
        // N s_19_1: branch s_19_0 b48 b20
        if s_19_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1s : i
        let s_20_0: i128 = 1;
        // D s_20_1: read-var size:i
        let s_20_1: i128 = fn_state.size;
        // D s_20_2: cmp-gt s_20_1 s_20_0
        let s_20_2: bool = ((s_20_1) > (s_20_0));
        // N s_20_3: assert s_20_2
        let s_20_3: () = assert!(s_20_2);
        // C s_20_4: const #() : ()
        let s_20_4: () = ();
        // S s_20_5: call HaveLRCPC3Ext(s_20_4)
        let s_20_5: bool = HaveLRCPC3Ext(state, tracer, s_20_4);
        // N s_20_6: branch s_20_5 b47 b21
        if s_20_5 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#20368 <= s_21_0
        fn_state.gs_20368 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#20368:u8
        let s_22_0: bool = fn_state.gs_20368;
        // N s_22_1: branch s_22_0 b46 b23
        if s_22_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#20369 <= s_23_0
        fn_state.gs_20369 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#20369:u8
        let s_24_0: bool = fn_state.gs_20369;
        // N s_24_1: branch s_24_0 b37 b25
        if s_24_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0s : i
        let s_25_0: i128 = 0;
        // D s_25_1: read-var value_name:bv
        let s_25_1: Bits = fn_state.value_name;
        // C s_25_2: const #1s : i64
        let s_25_2: i64 = 1;
        // C s_25_3: cast zx s_25_2 -> i
        let s_25_3: i128 = (i128::try_from(s_25_2).unwrap());
        // C s_25_4: const #7s : i
        let s_25_4: i128 = 7;
        // C s_25_5: add s_25_4 s_25_3
        let s_25_5: i128 = (s_25_4 + s_25_3);
        // D s_25_6: bit-extract s_25_1 s_25_0 s_25_5
        let s_25_6: Bits = (Bits::new(
            ((s_25_1) >> (s_25_0)).value(),
            u16::try_from(s_25_5).unwrap(),
        ));
        // D s_25_7: cast reint s_25_6 -> u8
        let s_25_7: u8 = (s_25_6.value() as u8);
        // C s_25_8: const #1s : i64
        let s_25_8: i64 = 1;
        // D s_25_9: cast zx s_25_7 -> bv
        let s_25_9: Bits = Bits::new(s_25_7 as u128, 8u16);
        // D s_25_10: read-var address:u64
        let s_25_10: u64 = fn_state.address;
        // D s_25_11: read-var accdesc:struct
        let s_25_11: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_25_12: read-var aligned:u8
        let s_25_12: bool = fn_state.aligned;
        // D s_25_13: call AArch64_MemSingle_set(s_25_10, s_25_8, s_25_11, s_25_12, s_25_9)
        let s_25_13: () = AArch64_MemSingle_set(
            state,
            tracer,
            s_25_10,
            s_25_8,
            s_25_11,
            s_25_12,
            s_25_9,
        );
        // N s_25_14: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #6u : u32
        let s_26_0: u32 = 6;
        // S s_26_1: call ConstrainUnpredictable(s_26_0)
        let s_26_1: u32 = ConstrainUnpredictable(state, tracer, s_26_0);
        // D s_26_2: write-var cshadow#329 <= s_26_1
        fn_state.cshadow_329 = s_26_1;
        // D s_26_3: read-var cshadow#329:u32
        let s_26_3: u32 = fn_state.cshadow_329;
        // C s_26_4: const #12u : u32
        let s_26_4: u32 = 12;
        // D s_26_5: cmp-eq s_26_3 s_26_4
        let s_26_5: bool = ((s_26_3) == (s_26_4));
        // N s_26_6: branch s_26_5 b36 b27
        if s_26_5 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var cshadow#329:u32
        let s_27_0: u32 = fn_state.cshadow_329;
        // C s_27_1: const #0u : u32
        let s_27_1: u32 = 0;
        // D s_27_2: cmp-eq s_27_0 s_27_1
        let s_27_2: bool = ((s_27_0) == (s_27_1));
        // D s_27_3: write-var gs#20374 <= s_27_2
        fn_state.gs_20374 = s_27_2;
        // N s_27_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#20374:u8
        let s_28_0: bool = fn_state.gs_20374;
        // N s_28_1: assert s_28_0
        let s_28_1: () = assert!(s_28_0);
        // D s_28_2: read-var cshadow#329:u32
        let s_28_2: u32 = fn_state.cshadow_329;
        // C s_28_3: const #0u : u32
        let s_28_3: u32 = 0;
        // D s_28_4: cmp-eq s_28_2 s_28_3
        let s_28_4: bool = ((s_28_2) == (s_28_3));
        // N s_28_5: branch s_28_4 b35 b29
        if s_28_4 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1s : i64
        let s_30_0: i64 = 1;
        // C s_30_1: const #1s : i
        let s_30_1: i128 = 1;
        // D s_30_2: read-var size:i
        let s_30_2: i128 = fn_state.size;
        // D s_30_3: sub s_30_2 s_30_1
        let s_30_3: i128 = ((s_30_2) - (s_30_1));
        // D s_30_4: cast reint s_30_3 -> i64
        let s_30_4: i64 = (s_30_3 as i64);
        // D s_30_5: write-var gs#20380 <= s_30_4
        fn_state.gs_20380 = s_30_4;
        // D s_30_6: write-var u#933 <= s_30_0
        fn_state.u_933 = s_30_0;
        // N s_30_7: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var u#933:i64
        let s_31_0: i64 = fn_state.u_933;
        // D s_31_1: read-var gs#20380:i64
        let s_31_1: i64 = fn_state.gs_20380;
        // D s_31_2: cmp-gt s_31_0 s_31_1
        let s_31_2: bool = ((s_31_0) > (s_31_1));
        // N s_31_3: branch s_31_2 b34 b32
        if s_31_2 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var address:u64
        let s_32_0: u64 = fn_state.address;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 64u16);
        // D s_32_2: read-var u#933:i64
        let s_32_2: i64 = fn_state.u_933;
        // D s_32_3: cast zx s_32_2 -> i
        let s_32_3: i128 = (i128::try_from(s_32_2).unwrap());
        // D s_32_4: cast cvt s_32_3 -> bv
        let s_32_4: Bits = Bits::new(s_32_3 as u128, 128);
        // D s_32_5: add s_32_1 s_32_4
        let s_32_5: Bits = (s_32_1 + s_32_4);
        // D s_32_6: cast reint s_32_5 -> u64
        let s_32_6: u64 = (s_32_5.value() as u64);
        // C s_32_7: const #8s : i
        let s_32_7: i128 = 8;
        // D s_32_8: read-var u#933:i64
        let s_32_8: i64 = fn_state.u_933;
        // D s_32_9: cast zx s_32_8 -> i
        let s_32_9: i128 = (i128::try_from(s_32_8).unwrap());
        // D s_32_10: mul s_32_7 s_32_9
        let s_32_10: i128 = ((s_32_7) * (s_32_9));
        // C s_32_11: const #8s : i
        let s_32_11: i128 = 8;
        // D s_32_12: read-var value_name:bv
        let s_32_12: Bits = fn_state.value_name;
        // D s_32_13: bit-extract s_32_12 s_32_10 s_32_11
        let s_32_13: Bits = (Bits::new(
            ((s_32_12) >> (s_32_10)).value(),
            u16::try_from(s_32_11).unwrap(),
        ));
        // D s_32_14: cast reint s_32_13 -> u8
        let s_32_14: u8 = (s_32_13.value() as u8);
        // C s_32_15: const #1s : i64
        let s_32_15: i64 = 1;
        // D s_32_16: cast zx s_32_14 -> bv
        let s_32_16: Bits = Bits::new(s_32_14 as u128, 8u16);
        // D s_32_17: read-var accdesc:struct
        let s_32_17: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_32_18: read-var aligned:u8
        let s_32_18: bool = fn_state.aligned;
        // D s_32_19: call AArch64_MemSingle_set(s_32_6, s_32_15, s_32_17, s_32_18, s_32_16)
        let s_32_19: () = AArch64_MemSingle_set(
            state,
            tracer,
            s_32_6,
            s_32_15,
            s_32_17,
            s_32_18,
            s_32_16,
        );
        // N s_32_20: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var u#933:i64
        let s_33_0: i64 = fn_state.u_933;
        // C s_33_1: const #1s : i64
        let s_33_1: i64 = 1;
        // D s_33_2: add s_33_0 s_33_1
        let s_33_2: i64 = (s_33_0 + s_33_1);
        // D s_33_3: write-var u#933 <= s_33_2
        fn_state.u_933 = s_33_2;
        // N s_33_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var aligned <= s_35_0
        fn_state.aligned = s_35_0;
        // N s_35_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#20374 <= s_36_0
        fn_state.gs_20374 = s_36_0;
        // N s_36_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var size:i
        let s_37_0: i128 = fn_state.size;
        // D s_37_1: call __id(s_37_0)
        let s_37_1: i128 = u__id(state, tracer, s_37_0);
        // C s_37_2: const #8s : i
        let s_37_2: i128 = 8;
        // D s_37_3: mul s_37_1 s_37_2
        let s_37_3: i128 = ((s_37_1) * (s_37_2));
        // D s_37_4: read-var halfsize:i
        let s_37_4: i128 = fn_state.halfsize;
        // D s_37_5: call __id(s_37_4)
        let s_37_5: i128 = u__id(state, tracer, s_37_4);
        // C s_37_6: const #8s : i
        let s_37_6: i128 = 8;
        // D s_37_7: mul s_37_5 s_37_6
        let s_37_7: i128 = ((s_37_5) * (s_37_6));
        // D s_37_8: read-var halfsize:i
        let s_37_8: i128 = fn_state.halfsize;
        // D s_37_9: call __id(s_37_8)
        let s_37_9: i128 = u__id(state, tracer, s_37_8);
        // C s_37_10: const #8s : i
        let s_37_10: i128 = 8;
        // D s_37_11: mul s_37_9 s_37_10
        let s_37_11: i128 = ((s_37_9) * (s_37_10));
        // D s_37_12: add s_37_7 s_37_11
        let s_37_12: i128 = (s_37_7 + s_37_11);
        // D s_37_13: cmp-eq s_37_3 s_37_12
        let s_37_13: bool = ((s_37_3) == (s_37_12));
        // N s_37_14: assert s_37_13
        let s_37_14: () = assert!(s_37_13);
        // D s_37_15: read-var value_in_name:bv
        let s_37_15: Bits = fn_state.value_in_name;
        // D s_37_16: size-of s_37_15
        let s_37_16: u16 = s_37_15.length();
        // D s_37_17: cast zx s_37_16 -> i
        let s_37_17: i128 = (i128::try_from(s_37_16).unwrap());
        // C s_37_18: const #1s : i
        let s_37_18: i128 = 1;
        // D s_37_19: sub s_37_17 s_37_18
        let s_37_19: i128 = ((s_37_17) - (s_37_18));
        // D s_37_20: read-var value_in_name:bv
        let s_37_20: Bits = fn_state.value_in_name;
        // D s_37_21: size-of s_37_20
        let s_37_21: u16 = s_37_20.length();
        // D s_37_22: cast zx s_37_21 -> i
        let s_37_22: i128 = (i128::try_from(s_37_21).unwrap());
        // C s_37_23: const #1s : i
        let s_37_23: i128 = 1;
        // D s_37_24: sub s_37_22 s_37_23
        let s_37_24: i128 = ((s_37_22) - (s_37_23));
        // D s_37_25: read-var halfsize:i
        let s_37_25: i128 = fn_state.halfsize;
        // D s_37_26: call __id(s_37_25)
        let s_37_26: i128 = u__id(state, tracer, s_37_25);
        // C s_37_27: const #8s : i
        let s_37_27: i128 = 8;
        // D s_37_28: mul s_37_26 s_37_27
        let s_37_28: i128 = ((s_37_26) * (s_37_27));
        // D s_37_29: sub s_37_24 s_37_28
        let s_37_29: i128 = ((s_37_24) - (s_37_28));
        // C s_37_30: const #1s : i
        let s_37_30: i128 = 1;
        // D s_37_31: add s_37_29 s_37_30
        let s_37_31: i128 = (s_37_29 + s_37_30);
        // D s_37_32: read-var value_name:bv
        let s_37_32: Bits = fn_state.value_name;
        // C s_37_33: const #1s : i64
        let s_37_33: i64 = 1;
        // C s_37_34: cast zx s_37_33 -> i
        let s_37_34: i128 = (i128::try_from(s_37_33).unwrap());
        // D s_37_35: sub s_37_19 s_37_31
        let s_37_35: i128 = ((s_37_19) - (s_37_31));
        // D s_37_36: add s_37_35 s_37_34
        let s_37_36: i128 = (s_37_35 + s_37_34);
        // D s_37_37: bit-extract s_37_32 s_37_31 s_37_36
        let s_37_37: Bits = (Bits::new(
            ((s_37_32) >> (s_37_31)).value(),
            u16::try_from(s_37_36).unwrap(),
        ));
        // D s_37_38: write-var u#930 <= s_37_37
        fn_state.u_930 = s_37_37;
        // D s_37_39: read-var value_in_name:bv
        let s_37_39: Bits = fn_state.value_in_name;
        // D s_37_40: size-of s_37_39
        let s_37_40: u16 = s_37_39.length();
        // D s_37_41: cast zx s_37_40 -> i
        let s_37_41: i128 = (i128::try_from(s_37_40).unwrap());
        // C s_37_42: const #1s : i
        let s_37_42: i128 = 1;
        // D s_37_43: sub s_37_41 s_37_42
        let s_37_43: i128 = ((s_37_41) - (s_37_42));
        // D s_37_44: read-var halfsize:i
        let s_37_44: i128 = fn_state.halfsize;
        // D s_37_45: call __id(s_37_44)
        let s_37_45: i128 = u__id(state, tracer, s_37_44);
        // C s_37_46: const #8s : i
        let s_37_46: i128 = 8;
        // D s_37_47: mul s_37_45 s_37_46
        let s_37_47: i128 = ((s_37_45) * (s_37_46));
        // D s_37_48: sub s_37_43 s_37_47
        let s_37_48: i128 = ((s_37_43) - (s_37_47));
        // D s_37_49: read-var value_in_name:bv
        let s_37_49: Bits = fn_state.value_in_name;
        // D s_37_50: size-of s_37_49
        let s_37_50: u16 = s_37_49.length();
        // D s_37_51: cast zx s_37_50 -> i
        let s_37_51: i128 = (i128::try_from(s_37_50).unwrap());
        // C s_37_52: const #1s : i
        let s_37_52: i128 = 1;
        // D s_37_53: sub s_37_51 s_37_52
        let s_37_53: i128 = ((s_37_51) - (s_37_52));
        // D s_37_54: read-var halfsize:i
        let s_37_54: i128 = fn_state.halfsize;
        // D s_37_55: call __id(s_37_54)
        let s_37_55: i128 = u__id(state, tracer, s_37_54);
        // C s_37_56: const #8s : i
        let s_37_56: i128 = 8;
        // D s_37_57: mul s_37_55 s_37_56
        let s_37_57: i128 = ((s_37_55) * (s_37_56));
        // D s_37_58: sub s_37_53 s_37_57
        let s_37_58: i128 = ((s_37_53) - (s_37_57));
        // D s_37_59: read-var halfsize:i
        let s_37_59: i128 = fn_state.halfsize;
        // D s_37_60: call __id(s_37_59)
        let s_37_60: i128 = u__id(state, tracer, s_37_59);
        // C s_37_61: const #8s : i
        let s_37_61: i128 = 8;
        // D s_37_62: mul s_37_60 s_37_61
        let s_37_62: i128 = ((s_37_60) * (s_37_61));
        // D s_37_63: sub s_37_58 s_37_62
        let s_37_63: i128 = ((s_37_58) - (s_37_62));
        // D s_37_64: cast reint s_37_63 -> i64
        let s_37_64: i64 = (s_37_63 as i64);
        // C s_37_65: const #1s : i
        let s_37_65: i128 = 1;
        // D s_37_66: cast zx s_37_64 -> i
        let s_37_66: i128 = (i128::try_from(s_37_64).unwrap());
        // D s_37_67: add s_37_66 s_37_65
        let s_37_67: i128 = (s_37_66 + s_37_65);
        // D s_37_68: cast reint s_37_67 -> i64
        let s_37_68: i64 = (s_37_67 as i64);
        // D s_37_69: cast zx s_37_68 -> i
        let s_37_69: i128 = (i128::try_from(s_37_68).unwrap());
        // D s_37_70: read-var value_name:bv
        let s_37_70: Bits = fn_state.value_name;
        // C s_37_71: const #1s : i64
        let s_37_71: i64 = 1;
        // C s_37_72: cast zx s_37_71 -> i
        let s_37_72: i128 = (i128::try_from(s_37_71).unwrap());
        // D s_37_73: sub s_37_48 s_37_69
        let s_37_73: i128 = ((s_37_48) - (s_37_69));
        // D s_37_74: add s_37_73 s_37_72
        let s_37_74: i128 = (s_37_73 + s_37_72);
        // D s_37_75: bit-extract s_37_70 s_37_69 s_37_74
        let s_37_75: Bits = (Bits::new(
            ((s_37_70) >> (s_37_69)).value(),
            u16::try_from(s_37_74).unwrap(),
        ));
        // D s_37_76: write-var u#931 <= s_37_75
        fn_state.u_931 = s_37_75;
        // C s_37_77: const #0s : i64
        let s_37_77: i64 = 0;
        // C s_37_78: const #1s : i
        let s_37_78: i128 = 1;
        // D s_37_79: read-var halfsize:i
        let s_37_79: i128 = fn_state.halfsize;
        // D s_37_80: sub s_37_79 s_37_78
        let s_37_80: i128 = ((s_37_79) - (s_37_78));
        // D s_37_81: cast reint s_37_80 -> i64
        let s_37_81: i64 = (s_37_80 as i64);
        // D s_37_82: write-var gs#20404 <= s_37_81
        fn_state.gs_20404 = s_37_81;
        // D s_37_83: write-var i <= s_37_77
        fn_state.i = s_37_77;
        // N s_37_84: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var i:i64
        let s_38_0: i64 = fn_state.i;
        // D s_38_1: read-var gs#20404:i64
        let s_38_1: i64 = fn_state.gs_20404;
        // D s_38_2: cmp-gt s_38_0 s_38_1
        let s_38_2: bool = ((s_38_0) > (s_38_1));
        // N s_38_3: branch s_38_2 b41 b39
        if s_38_2 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var address:u64
        let s_39_0: u64 = fn_state.address;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 64u16);
        // D s_39_2: read-var halfsize:i
        let s_39_2: i128 = fn_state.halfsize;
        // D s_39_3: cast cvt s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 128);
        // D s_39_4: add s_39_1 s_39_3
        let s_39_4: Bits = (s_39_1 + s_39_3);
        // D s_39_5: cast reint s_39_4 -> u64
        let s_39_5: u64 = (s_39_4.value() as u64);
        // D s_39_6: cast zx s_39_5 -> bv
        let s_39_6: Bits = Bits::new(s_39_5 as u128, 64u16);
        // D s_39_7: read-var i:i64
        let s_39_7: i64 = fn_state.i;
        // D s_39_8: cast zx s_39_7 -> i
        let s_39_8: i128 = (i128::try_from(s_39_7).unwrap());
        // D s_39_9: cast cvt s_39_8 -> bv
        let s_39_9: Bits = Bits::new(s_39_8 as u128, 128);
        // D s_39_10: add s_39_6 s_39_9
        let s_39_10: Bits = (s_39_6 + s_39_9);
        // D s_39_11: cast reint s_39_10 -> u64
        let s_39_11: u64 = (s_39_10.value() as u64);
        // C s_39_12: const #8s : i
        let s_39_12: i128 = 8;
        // D s_39_13: read-var i:i64
        let s_39_13: i64 = fn_state.i;
        // D s_39_14: cast zx s_39_13 -> i
        let s_39_14: i128 = (i128::try_from(s_39_13).unwrap());
        // D s_39_15: mul s_39_12 s_39_14
        let s_39_15: i128 = ((s_39_12) * (s_39_14));
        // C s_39_16: const #8s : i
        let s_39_16: i128 = 8;
        // D s_39_17: read-var u#930:bv
        let s_39_17: Bits = fn_state.u_930;
        // D s_39_18: bit-extract s_39_17 s_39_15 s_39_16
        let s_39_18: Bits = (Bits::new(
            ((s_39_17) >> (s_39_15)).value(),
            u16::try_from(s_39_16).unwrap(),
        ));
        // D s_39_19: cast reint s_39_18 -> u8
        let s_39_19: u8 = (s_39_18.value() as u8);
        // C s_39_20: const #1s : i64
        let s_39_20: i64 = 1;
        // D s_39_21: cast zx s_39_19 -> bv
        let s_39_21: Bits = Bits::new(s_39_19 as u128, 8u16);
        // D s_39_22: read-var accdesc:struct
        let s_39_22: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_39_23: read-var aligned:u8
        let s_39_23: bool = fn_state.aligned;
        // D s_39_24: call AArch64_MemSingle_set(s_39_11, s_39_20, s_39_22, s_39_23, s_39_21)
        let s_39_24: () = AArch64_MemSingle_set(
            state,
            tracer,
            s_39_11,
            s_39_20,
            s_39_22,
            s_39_23,
            s_39_21,
        );
        // N s_39_25: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var i:i64
        let s_40_0: i64 = fn_state.i;
        // C s_40_1: const #1s : i64
        let s_40_1: i64 = 1;
        // D s_40_2: add s_40_0 s_40_1
        let s_40_2: i64 = (s_40_0 + s_40_1);
        // D s_40_3: write-var i <= s_40_2
        fn_state.i = s_40_2;
        // N s_40_4: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0s : i64
        let s_41_0: i64 = 0;
        // C s_41_1: const #1s : i
        let s_41_1: i128 = 1;
        // D s_41_2: read-var halfsize:i
        let s_41_2: i128 = fn_state.halfsize;
        // D s_41_3: sub s_41_2 s_41_1
        let s_41_3: i128 = ((s_41_2) - (s_41_1));
        // D s_41_4: cast reint s_41_3 -> i64
        let s_41_4: i64 = (s_41_3 as i64);
        // D s_41_5: write-var gs#20417 <= s_41_4
        fn_state.gs_20417 = s_41_4;
        // D s_41_6: write-var u#932 <= s_41_0
        fn_state.u_932 = s_41_0;
        // N s_41_7: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var u#932:i64
        let s_42_0: i64 = fn_state.u_932;
        // D s_42_1: read-var gs#20417:i64
        let s_42_1: i64 = fn_state.gs_20417;
        // D s_42_2: cmp-gt s_42_0 s_42_1
        let s_42_2: bool = ((s_42_0) > (s_42_1));
        // N s_42_3: branch s_42_2 b45 b43
        if s_42_2 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var address:u64
        let s_43_0: u64 = fn_state.address;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 64u16);
        // D s_43_2: read-var u#932:i64
        let s_43_2: i64 = fn_state.u_932;
        // D s_43_3: cast zx s_43_2 -> i
        let s_43_3: i128 = (i128::try_from(s_43_2).unwrap());
        // D s_43_4: cast cvt s_43_3 -> bv
        let s_43_4: Bits = Bits::new(s_43_3 as u128, 128);
        // D s_43_5: add s_43_1 s_43_4
        let s_43_5: Bits = (s_43_1 + s_43_4);
        // D s_43_6: cast reint s_43_5 -> u64
        let s_43_6: u64 = (s_43_5.value() as u64);
        // C s_43_7: const #8s : i
        let s_43_7: i128 = 8;
        // D s_43_8: read-var u#932:i64
        let s_43_8: i64 = fn_state.u_932;
        // D s_43_9: cast zx s_43_8 -> i
        let s_43_9: i128 = (i128::try_from(s_43_8).unwrap());
        // D s_43_10: mul s_43_7 s_43_9
        let s_43_10: i128 = ((s_43_7) * (s_43_9));
        // C s_43_11: const #8s : i
        let s_43_11: i128 = 8;
        // D s_43_12: read-var u#931:bv
        let s_43_12: Bits = fn_state.u_931;
        // D s_43_13: bit-extract s_43_12 s_43_10 s_43_11
        let s_43_13: Bits = (Bits::new(
            ((s_43_12) >> (s_43_10)).value(),
            u16::try_from(s_43_11).unwrap(),
        ));
        // D s_43_14: cast reint s_43_13 -> u8
        let s_43_14: u8 = (s_43_13.value() as u8);
        // C s_43_15: const #1s : i64
        let s_43_15: i64 = 1;
        // D s_43_16: cast zx s_43_14 -> bv
        let s_43_16: Bits = Bits::new(s_43_14 as u128, 8u16);
        // D s_43_17: read-var accdesc:struct
        let s_43_17: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_43_18: read-var aligned:u8
        let s_43_18: bool = fn_state.aligned;
        // D s_43_19: call AArch64_MemSingle_set(s_43_6, s_43_15, s_43_17, s_43_18, s_43_16)
        let s_43_19: () = AArch64_MemSingle_set(
            state,
            tracer,
            s_43_6,
            s_43_15,
            s_43_17,
            s_43_18,
            s_43_16,
        );
        // N s_43_20: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var u#932:i64
        let s_44_0: i64 = fn_state.u_932;
        // C s_44_1: const #1s : i64
        let s_44_1: i64 = 1;
        // D s_44_2: add s_44_0 s_44_1
        let s_44_2: i64 = (s_44_0 + s_44_1);
        // D s_44_3: write-var u#932 <= s_44_2
        fn_state.u_932 = s_44_2;
        // N s_44_4: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var highestAddressfirst:u8
        let s_46_0: bool = fn_state.highestAddressfirst;
        // D s_46_1: write-var gs#20369 <= s_46_0
        fn_state.gs_20369 = s_46_0;
        // N s_46_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var ispair:u8
        let s_47_0: bool = fn_state.ispair;
        // D s_47_1: write-var gs#20368 <= s_47_0
        fn_state.gs_20368 = s_47_0;
        // N s_47_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var size:i
        let s_48_0: i128 = fn_state.size;
        // D s_48_1: call __id(s_48_0)
        let s_48_1: i128 = u__id(state, tracer, s_48_0);
        // C s_48_2: const #1s : i
        let s_48_2: i128 = 1;
        // D s_48_3: cmp-eq s_48_1 s_48_2
        let s_48_3: bool = ((s_48_1) == (s_48_2));
        // N s_48_4: branch s_48_3 b60 b49
        if s_48_3 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var size:i
        let s_49_0: i128 = fn_state.size;
        // D s_49_1: call __id(s_49_0)
        let s_49_1: i128 = u__id(state, tracer, s_49_0);
        // C s_49_2: const #2s : i
        let s_49_2: i128 = 2;
        // D s_49_3: cmp-eq s_49_1 s_49_2
        let s_49_3: bool = ((s_49_1) == (s_49_2));
        // D s_49_4: write-var gs#20426 <= s_49_3
        fn_state.gs_20426 = s_49_3;
        // N s_49_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#20426:u8
        let s_50_0: bool = fn_state.gs_20426;
        // N s_50_1: branch s_50_0 b59 b51
        if s_50_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var size:i
        let s_51_0: i128 = fn_state.size;
        // D s_51_1: call __id(s_51_0)
        let s_51_1: i128 = u__id(state, tracer, s_51_0);
        // C s_51_2: const #4s : i
        let s_51_2: i128 = 4;
        // D s_51_3: cmp-eq s_51_1 s_51_2
        let s_51_3: bool = ((s_51_1) == (s_51_2));
        // D s_51_4: write-var gs#20428 <= s_51_3
        fn_state.gs_20428 = s_51_3;
        // N s_51_5: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#20428:u8
        let s_52_0: bool = fn_state.gs_20428;
        // N s_52_1: branch s_52_0 b58 b53
        if s_52_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var size:i
        let s_53_0: i128 = fn_state.size;
        // D s_53_1: call __id(s_53_0)
        let s_53_1: i128 = u__id(state, tracer, s_53_0);
        // C s_53_2: const #8s : i
        let s_53_2: i128 = 8;
        // D s_53_3: cmp-eq s_53_1 s_53_2
        let s_53_3: bool = ((s_53_1) == (s_53_2));
        // D s_53_4: write-var gs#20430 <= s_53_3
        fn_state.gs_20430 = s_53_3;
        // N s_53_5: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#20430:u8
        let s_54_0: bool = fn_state.gs_20430;
        // N s_54_1: branch s_54_0 b57 b55
        if s_54_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var size:i
        let s_55_0: i128 = fn_state.size;
        // D s_55_1: call __id(s_55_0)
        let s_55_1: i128 = u__id(state, tracer, s_55_0);
        // C s_55_2: const #16s : i
        let s_55_2: i128 = 16;
        // D s_55_3: cmp-eq s_55_1 s_55_2
        let s_55_3: bool = ((s_55_1) == (s_55_2));
        // D s_55_4: write-var gs#20432 <= s_55_3
        fn_state.gs_20432 = s_55_3;
        // N s_55_5: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#20432:u8
        let s_56_0: bool = fn_state.gs_20432;
        // N s_56_1: assert s_56_0
        let s_56_1: () = assert!(s_56_0);
        // D s_56_2: read-var size:i
        let s_56_2: i128 = fn_state.size;
        // D s_56_3: cast reint s_56_2 -> i64
        let s_56_3: i64 = (s_56_2 as i64);
        // D s_56_4: read-var address:u64
        let s_56_4: u64 = fn_state.address;
        // D s_56_5: read-var accdesc:struct
        let s_56_5: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_56_6: read-var aligned:u8
        let s_56_6: bool = fn_state.aligned;
        // D s_56_7: read-var ispair:u8
        let s_56_7: bool = fn_state.ispair;
        // D s_56_8: read-var value_name:bv
        let s_56_8: Bits = fn_state.value_name;
        // D s_56_9: call AArch64_MemSingle_set__1(s_56_4, s_56_3, s_56_5, s_56_6, s_56_7, s_56_8)
        let s_56_9: () = AArch64_MemSingle_set__1(
            state,
            tracer,
            s_56_4,
            s_56_3,
            s_56_5,
            s_56_6,
            s_56_7,
            s_56_8,
        );
        // N s_56_10: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#20432 <= s_57_0
        fn_state.gs_20432 = s_57_0;
        // N s_57_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#20430 <= s_58_0
        fn_state.gs_20430 = s_58_0;
        // N s_58_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #1u : u8
        let s_59_0: bool = true;
        // D s_59_1: write-var gs#20428 <= s_59_0
        fn_state.gs_20428 = s_59_0;
        // N s_59_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#20426 <= s_60_0
        fn_state.gs_20426 = s_60_0;
        // N s_60_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0s : i
        let s_61_0: i128 = 0;
        // D s_61_1: read-var halfsize:i
        let s_61_1: i128 = fn_state.halfsize;
        // D s_61_2: cmp-gt s_61_1 s_61_0
        let s_61_2: bool = ((s_61_1) > (s_61_0));
        // N s_61_3: assert s_61_2
        let s_61_3: () = assert!(s_61_2);
        // D s_61_4: read-var size:i
        let s_61_4: i128 = fn_state.size;
        // D s_61_5: call __id(s_61_4)
        let s_61_5: i128 = u__id(state, tracer, s_61_4);
        // C s_61_6: const #8s : i
        let s_61_6: i128 = 8;
        // D s_61_7: mul s_61_5 s_61_6
        let s_61_7: i128 = ((s_61_5) * (s_61_6));
        // D s_61_8: read-var halfsize:i
        let s_61_8: i128 = fn_state.halfsize;
        // D s_61_9: call __id(s_61_8)
        let s_61_9: i128 = u__id(state, tracer, s_61_8);
        // C s_61_10: const #8s : i
        let s_61_10: i128 = 8;
        // D s_61_11: mul s_61_9 s_61_10
        let s_61_11: i128 = ((s_61_9) * (s_61_10));
        // D s_61_12: read-var halfsize:i
        let s_61_12: i128 = fn_state.halfsize;
        // D s_61_13: call __id(s_61_12)
        let s_61_13: i128 = u__id(state, tracer, s_61_12);
        // C s_61_14: const #8s : i
        let s_61_14: i128 = 8;
        // D s_61_15: mul s_61_13 s_61_14
        let s_61_15: i128 = ((s_61_13) * (s_61_14));
        // D s_61_16: add s_61_11 s_61_15
        let s_61_16: i128 = (s_61_11 + s_61_15);
        // D s_61_17: cmp-eq s_61_7 s_61_16
        let s_61_17: bool = ((s_61_7) == (s_61_16));
        // N s_61_18: assert s_61_17
        let s_61_18: () = assert!(s_61_17);
        // D s_61_19: read-var value_in_name:bv
        let s_61_19: Bits = fn_state.value_in_name;
        // D s_61_20: size-of s_61_19
        let s_61_20: u16 = s_61_19.length();
        // D s_61_21: cast zx s_61_20 -> i
        let s_61_21: i128 = (i128::try_from(s_61_20).unwrap());
        // C s_61_22: const #1s : i
        let s_61_22: i128 = 1;
        // D s_61_23: sub s_61_21 s_61_22
        let s_61_23: i128 = ((s_61_21) - (s_61_22));
        // D s_61_24: read-var value_in_name:bv
        let s_61_24: Bits = fn_state.value_in_name;
        // D s_61_25: size-of s_61_24
        let s_61_25: u16 = s_61_24.length();
        // D s_61_26: cast zx s_61_25 -> i
        let s_61_26: i128 = (i128::try_from(s_61_25).unwrap());
        // C s_61_27: const #1s : i
        let s_61_27: i128 = 1;
        // D s_61_28: sub s_61_26 s_61_27
        let s_61_28: i128 = ((s_61_26) - (s_61_27));
        // D s_61_29: read-var halfsize:i
        let s_61_29: i128 = fn_state.halfsize;
        // D s_61_30: call __id(s_61_29)
        let s_61_30: i128 = u__id(state, tracer, s_61_29);
        // C s_61_31: const #8s : i
        let s_61_31: i128 = 8;
        // D s_61_32: mul s_61_30 s_61_31
        let s_61_32: i128 = ((s_61_30) * (s_61_31));
        // D s_61_33: sub s_61_28 s_61_32
        let s_61_33: i128 = ((s_61_28) - (s_61_32));
        // C s_61_34: const #1s : i
        let s_61_34: i128 = 1;
        // D s_61_35: add s_61_33 s_61_34
        let s_61_35: i128 = (s_61_33 + s_61_34);
        // D s_61_36: read-var value_name:bv
        let s_61_36: Bits = fn_state.value_name;
        // C s_61_37: const #1s : i64
        let s_61_37: i64 = 1;
        // C s_61_38: cast zx s_61_37 -> i
        let s_61_38: i128 = (i128::try_from(s_61_37).unwrap());
        // D s_61_39: sub s_61_23 s_61_35
        let s_61_39: i128 = ((s_61_23) - (s_61_35));
        // D s_61_40: add s_61_39 s_61_38
        let s_61_40: i128 = (s_61_39 + s_61_38);
        // D s_61_41: bit-extract s_61_36 s_61_35 s_61_40
        let s_61_41: Bits = (Bits::new(
            ((s_61_36) >> (s_61_35)).value(),
            u16::try_from(s_61_40).unwrap(),
        ));
        // D s_61_42: write-var u#928 <= s_61_41
        fn_state.u_928 = s_61_41;
        // D s_61_43: read-var value_in_name:bv
        let s_61_43: Bits = fn_state.value_in_name;
        // D s_61_44: size-of s_61_43
        let s_61_44: u16 = s_61_43.length();
        // D s_61_45: cast zx s_61_44 -> i
        let s_61_45: i128 = (i128::try_from(s_61_44).unwrap());
        // C s_61_46: const #1s : i
        let s_61_46: i128 = 1;
        // D s_61_47: sub s_61_45 s_61_46
        let s_61_47: i128 = ((s_61_45) - (s_61_46));
        // D s_61_48: read-var halfsize:i
        let s_61_48: i128 = fn_state.halfsize;
        // D s_61_49: call __id(s_61_48)
        let s_61_49: i128 = u__id(state, tracer, s_61_48);
        // C s_61_50: const #8s : i
        let s_61_50: i128 = 8;
        // D s_61_51: mul s_61_49 s_61_50
        let s_61_51: i128 = ((s_61_49) * (s_61_50));
        // D s_61_52: sub s_61_47 s_61_51
        let s_61_52: i128 = ((s_61_47) - (s_61_51));
        // D s_61_53: read-var value_in_name:bv
        let s_61_53: Bits = fn_state.value_in_name;
        // D s_61_54: size-of s_61_53
        let s_61_54: u16 = s_61_53.length();
        // D s_61_55: cast zx s_61_54 -> i
        let s_61_55: i128 = (i128::try_from(s_61_54).unwrap());
        // C s_61_56: const #1s : i
        let s_61_56: i128 = 1;
        // D s_61_57: sub s_61_55 s_61_56
        let s_61_57: i128 = ((s_61_55) - (s_61_56));
        // D s_61_58: read-var halfsize:i
        let s_61_58: i128 = fn_state.halfsize;
        // D s_61_59: call __id(s_61_58)
        let s_61_59: i128 = u__id(state, tracer, s_61_58);
        // C s_61_60: const #8s : i
        let s_61_60: i128 = 8;
        // D s_61_61: mul s_61_59 s_61_60
        let s_61_61: i128 = ((s_61_59) * (s_61_60));
        // D s_61_62: sub s_61_57 s_61_61
        let s_61_62: i128 = ((s_61_57) - (s_61_61));
        // D s_61_63: read-var halfsize:i
        let s_61_63: i128 = fn_state.halfsize;
        // D s_61_64: call __id(s_61_63)
        let s_61_64: i128 = u__id(state, tracer, s_61_63);
        // C s_61_65: const #8s : i
        let s_61_65: i128 = 8;
        // D s_61_66: mul s_61_64 s_61_65
        let s_61_66: i128 = ((s_61_64) * (s_61_65));
        // D s_61_67: sub s_61_62 s_61_66
        let s_61_67: i128 = ((s_61_62) - (s_61_66));
        // D s_61_68: cast reint s_61_67 -> i64
        let s_61_68: i64 = (s_61_67 as i64);
        // C s_61_69: const #1s : i
        let s_61_69: i128 = 1;
        // D s_61_70: cast zx s_61_68 -> i
        let s_61_70: i128 = (i128::try_from(s_61_68).unwrap());
        // D s_61_71: add s_61_70 s_61_69
        let s_61_71: i128 = (s_61_70 + s_61_69);
        // D s_61_72: cast reint s_61_71 -> i64
        let s_61_72: i64 = (s_61_71 as i64);
        // D s_61_73: cast zx s_61_72 -> i
        let s_61_73: i128 = (i128::try_from(s_61_72).unwrap());
        // D s_61_74: read-var value_name:bv
        let s_61_74: Bits = fn_state.value_name;
        // C s_61_75: const #1s : i64
        let s_61_75: i64 = 1;
        // C s_61_76: cast zx s_61_75 -> i
        let s_61_76: i128 = (i128::try_from(s_61_75).unwrap());
        // D s_61_77: sub s_61_52 s_61_73
        let s_61_77: i128 = ((s_61_52) - (s_61_73));
        // D s_61_78: add s_61_77 s_61_76
        let s_61_78: i128 = (s_61_77 + s_61_76);
        // D s_61_79: bit-extract s_61_74 s_61_73 s_61_78
        let s_61_79: Bits = (Bits::new(
            ((s_61_74) >> (s_61_73)).value(),
            u16::try_from(s_61_78).unwrap(),
        ));
        // D s_61_80: write-var u#929 <= s_61_79
        fn_state.u_929 = s_61_79;
        // C s_61_81: const #() : ()
        let s_61_81: () = ();
        // S s_61_82: call HaveLRCPC3Ext(s_61_81)
        let s_61_82: bool = HaveLRCPC3Ext(state, tracer, s_61_81);
        // N s_61_83: branch s_61_82 b92 b62
        if s_61_82 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#20452 <= s_62_0
        fn_state.gs_20452 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#20452:u8
        let s_63_0: bool = fn_state.gs_20452;
        // N s_63_1: branch s_63_0 b78 b64
        if s_63_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var halfsize:i
        let s_64_0: i128 = fn_state.halfsize;
        // D s_64_1: call __id(s_64_0)
        let s_64_1: i128 = u__id(state, tracer, s_64_0);
        // C s_64_2: const #1s : i
        let s_64_2: i128 = 1;
        // D s_64_3: cmp-eq s_64_1 s_64_2
        let s_64_3: bool = ((s_64_1) == (s_64_2));
        // N s_64_4: branch s_64_3 b77 b65
        if s_64_3 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var halfsize:i
        let s_65_0: i128 = fn_state.halfsize;
        // D s_65_1: call __id(s_65_0)
        let s_65_1: i128 = u__id(state, tracer, s_65_0);
        // C s_65_2: const #2s : i
        let s_65_2: i128 = 2;
        // D s_65_3: cmp-eq s_65_1 s_65_2
        let s_65_3: bool = ((s_65_1) == (s_65_2));
        // D s_65_4: write-var gs#20455 <= s_65_3
        fn_state.gs_20455 = s_65_3;
        // N s_65_5: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#20455:u8
        let s_66_0: bool = fn_state.gs_20455;
        // N s_66_1: branch s_66_0 b76 b67
        if s_66_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var halfsize:i
        let s_67_0: i128 = fn_state.halfsize;
        // D s_67_1: call __id(s_67_0)
        let s_67_1: i128 = u__id(state, tracer, s_67_0);
        // C s_67_2: const #4s : i
        let s_67_2: i128 = 4;
        // D s_67_3: cmp-eq s_67_1 s_67_2
        let s_67_3: bool = ((s_67_1) == (s_67_2));
        // D s_67_4: write-var gs#20457 <= s_67_3
        fn_state.gs_20457 = s_67_3;
        // N s_67_5: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#20457:u8
        let s_68_0: bool = fn_state.gs_20457;
        // N s_68_1: branch s_68_0 b75 b69
        if s_68_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var halfsize:i
        let s_69_0: i128 = fn_state.halfsize;
        // D s_69_1: call __id(s_69_0)
        let s_69_1: i128 = u__id(state, tracer, s_69_0);
        // C s_69_2: const #8s : i
        let s_69_2: i128 = 8;
        // D s_69_3: cmp-eq s_69_1 s_69_2
        let s_69_3: bool = ((s_69_1) == (s_69_2));
        // D s_69_4: write-var gs#20459 <= s_69_3
        fn_state.gs_20459 = s_69_3;
        // N s_69_5: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#20459:u8
        let s_70_0: bool = fn_state.gs_20459;
        // N s_70_1: branch s_70_0 b74 b71
        if s_70_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var halfsize:i
        let s_71_0: i128 = fn_state.halfsize;
        // D s_71_1: call __id(s_71_0)
        let s_71_1: i128 = u__id(state, tracer, s_71_0);
        // C s_71_2: const #16s : i
        let s_71_2: i128 = 16;
        // D s_71_3: cmp-eq s_71_1 s_71_2
        let s_71_3: bool = ((s_71_1) == (s_71_2));
        // D s_71_4: write-var gs#20461 <= s_71_3
        fn_state.gs_20461 = s_71_3;
        // N s_71_5: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#20461:u8
        let s_72_0: bool = fn_state.gs_20461;
        // N s_72_1: assert s_72_0
        let s_72_1: () = assert!(s_72_0);
        // D s_72_2: read-var halfsize:i
        let s_72_2: i128 = fn_state.halfsize;
        // D s_72_3: cast reint s_72_2 -> i64
        let s_72_3: i64 = (s_72_2 as i64);
        // D s_72_4: read-var address:u64
        let s_72_4: u64 = fn_state.address;
        // D s_72_5: read-var accdesc:struct
        let s_72_5: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_72_6: read-var aligned:u8
        let s_72_6: bool = fn_state.aligned;
        // C s_72_7: const #0u : u8
        let s_72_7: bool = false;
        // D s_72_8: read-var u#929:bv
        let s_72_8: Bits = fn_state.u_929;
        // D s_72_9: call AArch64_MemSingle_set__1(s_72_4, s_72_3, s_72_5, s_72_6, s_72_7, s_72_8)
        let s_72_9: () = AArch64_MemSingle_set__1(
            state,
            tracer,
            s_72_4,
            s_72_3,
            s_72_5,
            s_72_6,
            s_72_7,
            s_72_8,
        );
        // N s_72_10: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var address:u64
        let s_73_0: u64 = fn_state.address;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 64u16);
        // D s_73_2: read-var halfsize:i
        let s_73_2: i128 = fn_state.halfsize;
        // D s_73_3: cast cvt s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 128);
        // D s_73_4: add s_73_1 s_73_3
        let s_73_4: Bits = (s_73_1 + s_73_3);
        // D s_73_5: cast reint s_73_4 -> u64
        let s_73_5: u64 = (s_73_4.value() as u64);
        // D s_73_6: read-var halfsize:i
        let s_73_6: i128 = fn_state.halfsize;
        // D s_73_7: cast reint s_73_6 -> i64
        let s_73_7: i64 = (s_73_6 as i64);
        // D s_73_8: read-var accdesc:struct
        let s_73_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_73_9: read-var aligned:u8
        let s_73_9: bool = fn_state.aligned;
        // C s_73_10: const #0u : u8
        let s_73_10: bool = false;
        // D s_73_11: read-var u#928:bv
        let s_73_11: Bits = fn_state.u_928;
        // D s_73_12: call AArch64_MemSingle_set__1(s_73_5, s_73_7, s_73_8, s_73_9, s_73_10, s_73_11)
        let s_73_12: () = AArch64_MemSingle_set__1(
            state,
            tracer,
            s_73_5,
            s_73_7,
            s_73_8,
            s_73_9,
            s_73_10,
            s_73_11,
        );
        // N s_73_13: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #1u : u8
        let s_74_0: bool = true;
        // D s_74_1: write-var gs#20461 <= s_74_0
        fn_state.gs_20461 = s_74_0;
        // N s_74_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#20459 <= s_75_0
        fn_state.gs_20459 = s_75_0;
        // N s_75_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #1u : u8
        let s_76_0: bool = true;
        // D s_76_1: write-var gs#20457 <= s_76_0
        fn_state.gs_20457 = s_76_0;
        // N s_76_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #1u : u8
        let s_77_0: bool = true;
        // D s_77_1: write-var gs#20455 <= s_77_0
        fn_state.gs_20455 = s_77_0;
        // N s_77_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var halfsize:i
        let s_78_0: i128 = fn_state.halfsize;
        // D s_78_1: call __id(s_78_0)
        let s_78_1: i128 = u__id(state, tracer, s_78_0);
        // C s_78_2: const #1s : i
        let s_78_2: i128 = 1;
        // D s_78_3: cmp-eq s_78_1 s_78_2
        let s_78_3: bool = ((s_78_1) == (s_78_2));
        // N s_78_4: branch s_78_3 b91 b79
        if s_78_3 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var halfsize:i
        let s_79_0: i128 = fn_state.halfsize;
        // D s_79_1: call __id(s_79_0)
        let s_79_1: i128 = u__id(state, tracer, s_79_0);
        // C s_79_2: const #2s : i
        let s_79_2: i128 = 2;
        // D s_79_3: cmp-eq s_79_1 s_79_2
        let s_79_3: bool = ((s_79_1) == (s_79_2));
        // D s_79_4: write-var gs#20466 <= s_79_3
        fn_state.gs_20466 = s_79_3;
        // N s_79_5: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#20466:u8
        let s_80_0: bool = fn_state.gs_20466;
        // N s_80_1: branch s_80_0 b90 b81
        if s_80_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var halfsize:i
        let s_81_0: i128 = fn_state.halfsize;
        // D s_81_1: call __id(s_81_0)
        let s_81_1: i128 = u__id(state, tracer, s_81_0);
        // C s_81_2: const #4s : i
        let s_81_2: i128 = 4;
        // D s_81_3: cmp-eq s_81_1 s_81_2
        let s_81_3: bool = ((s_81_1) == (s_81_2));
        // D s_81_4: write-var gs#20468 <= s_81_3
        fn_state.gs_20468 = s_81_3;
        // N s_81_5: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#20468:u8
        let s_82_0: bool = fn_state.gs_20468;
        // N s_82_1: branch s_82_0 b89 b83
        if s_82_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var halfsize:i
        let s_83_0: i128 = fn_state.halfsize;
        // D s_83_1: call __id(s_83_0)
        let s_83_1: i128 = u__id(state, tracer, s_83_0);
        // C s_83_2: const #8s : i
        let s_83_2: i128 = 8;
        // D s_83_3: cmp-eq s_83_1 s_83_2
        let s_83_3: bool = ((s_83_1) == (s_83_2));
        // D s_83_4: write-var gs#20470 <= s_83_3
        fn_state.gs_20470 = s_83_3;
        // N s_83_5: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#20470:u8
        let s_84_0: bool = fn_state.gs_20470;
        // N s_84_1: branch s_84_0 b88 b85
        if s_84_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var halfsize:i
        let s_85_0: i128 = fn_state.halfsize;
        // D s_85_1: call __id(s_85_0)
        let s_85_1: i128 = u__id(state, tracer, s_85_0);
        // C s_85_2: const #16s : i
        let s_85_2: i128 = 16;
        // D s_85_3: cmp-eq s_85_1 s_85_2
        let s_85_3: bool = ((s_85_1) == (s_85_2));
        // D s_85_4: write-var gs#20472 <= s_85_3
        fn_state.gs_20472 = s_85_3;
        // N s_85_5: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#20472:u8
        let s_86_0: bool = fn_state.gs_20472;
        // N s_86_1: assert s_86_0
        let s_86_1: () = assert!(s_86_0);
        // D s_86_2: read-var address:u64
        let s_86_2: u64 = fn_state.address;
        // D s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 64u16);
        // D s_86_4: read-var halfsize:i
        let s_86_4: i128 = fn_state.halfsize;
        // D s_86_5: cast cvt s_86_4 -> bv
        let s_86_5: Bits = Bits::new(s_86_4 as u128, 128);
        // D s_86_6: add s_86_3 s_86_5
        let s_86_6: Bits = (s_86_3 + s_86_5);
        // D s_86_7: cast reint s_86_6 -> u64
        let s_86_7: u64 = (s_86_6.value() as u64);
        // D s_86_8: read-var halfsize:i
        let s_86_8: i128 = fn_state.halfsize;
        // D s_86_9: cast reint s_86_8 -> i64
        let s_86_9: i64 = (s_86_8 as i64);
        // D s_86_10: read-var accdesc:struct
        let s_86_10: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_86_11: read-var aligned:u8
        let s_86_11: bool = fn_state.aligned;
        // C s_86_12: const #0u : u8
        let s_86_12: bool = false;
        // D s_86_13: read-var u#928:bv
        let s_86_13: Bits = fn_state.u_928;
        // D s_86_14: call AArch64_MemSingle_set__1(s_86_7, s_86_9, s_86_10, s_86_11, s_86_12, s_86_13)
        let s_86_14: () = AArch64_MemSingle_set__1(
            state,
            tracer,
            s_86_7,
            s_86_9,
            s_86_10,
            s_86_11,
            s_86_12,
            s_86_13,
        );
        // N s_86_15: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var halfsize:i
        let s_87_0: i128 = fn_state.halfsize;
        // D s_87_1: cast reint s_87_0 -> i64
        let s_87_1: i64 = (s_87_0 as i64);
        // D s_87_2: read-var address:u64
        let s_87_2: u64 = fn_state.address;
        // D s_87_3: read-var accdesc:struct
        let s_87_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_87_4: read-var aligned:u8
        let s_87_4: bool = fn_state.aligned;
        // C s_87_5: const #0u : u8
        let s_87_5: bool = false;
        // D s_87_6: read-var u#929:bv
        let s_87_6: Bits = fn_state.u_929;
        // D s_87_7: call AArch64_MemSingle_set__1(s_87_2, s_87_1, s_87_3, s_87_4, s_87_5, s_87_6)
        let s_87_7: () = AArch64_MemSingle_set__1(
            state,
            tracer,
            s_87_2,
            s_87_1,
            s_87_3,
            s_87_4,
            s_87_5,
            s_87_6,
        );
        // N s_87_8: return
        return;
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #1u : u8
        let s_88_0: bool = true;
        // D s_88_1: write-var gs#20472 <= s_88_0
        fn_state.gs_20472 = s_88_0;
        // N s_88_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #1u : u8
        let s_89_0: bool = true;
        // D s_89_1: write-var gs#20470 <= s_89_0
        fn_state.gs_20470 = s_89_0;
        // N s_89_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #1u : u8
        let s_90_0: bool = true;
        // D s_90_1: write-var gs#20468 <= s_90_0
        fn_state.gs_20468 = s_90_0;
        // N s_90_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #1u : u8
        let s_91_0: bool = true;
        // D s_91_1: write-var gs#20466 <= s_91_0
        fn_state.gs_20466 = s_91_0;
        // N s_91_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var highestAddressfirst:u8
        let s_92_0: bool = fn_state.highestAddressfirst;
        // D s_92_1: write-var gs#20452 <= s_92_0
        fn_state.gs_20452 = s_92_0;
        // N s_92_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var aligned:u8
        let s_93_0: bool = fn_state.aligned;
        // D s_93_1: write-var gs#20365 <= s_93_0
        fn_state.gs_20365 = s_93_0;
        // N s_93_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var size:i
        let s_94_0: i128 = fn_state.size;
        // D s_94_1: call __id(s_94_0)
        let s_94_1: i128 = u__id(state, tracer, s_94_0);
        // C s_94_2: const #1s : i
        let s_94_2: i128 = 1;
        // D s_94_3: cmp-eq s_94_1 s_94_2
        let s_94_3: bool = ((s_94_1) == (s_94_2));
        // N s_94_4: branch s_94_3 b106 b95
        if s_94_3 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var size:i
        let s_95_0: i128 = fn_state.size;
        // D s_95_1: call __id(s_95_0)
        let s_95_1: i128 = u__id(state, tracer, s_95_0);
        // C s_95_2: const #2s : i
        let s_95_2: i128 = 2;
        // D s_95_3: cmp-eq s_95_1 s_95_2
        let s_95_3: bool = ((s_95_1) == (s_95_2));
        // D s_95_4: write-var gs#20477 <= s_95_3
        fn_state.gs_20477 = s_95_3;
        // N s_95_5: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#20477:u8
        let s_96_0: bool = fn_state.gs_20477;
        // N s_96_1: branch s_96_0 b105 b97
        if s_96_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var size:i
        let s_97_0: i128 = fn_state.size;
        // D s_97_1: call __id(s_97_0)
        let s_97_1: i128 = u__id(state, tracer, s_97_0);
        // C s_97_2: const #4s : i
        let s_97_2: i128 = 4;
        // D s_97_3: cmp-eq s_97_1 s_97_2
        let s_97_3: bool = ((s_97_1) == (s_97_2));
        // D s_97_4: write-var gs#20479 <= s_97_3
        fn_state.gs_20479 = s_97_3;
        // N s_97_5: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#20479:u8
        let s_98_0: bool = fn_state.gs_20479;
        // N s_98_1: branch s_98_0 b104 b99
        if s_98_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var size:i
        let s_99_0: i128 = fn_state.size;
        // D s_99_1: call __id(s_99_0)
        let s_99_1: i128 = u__id(state, tracer, s_99_0);
        // C s_99_2: const #8s : i
        let s_99_2: i128 = 8;
        // D s_99_3: cmp-eq s_99_1 s_99_2
        let s_99_3: bool = ((s_99_1) == (s_99_2));
        // D s_99_4: write-var gs#20481 <= s_99_3
        fn_state.gs_20481 = s_99_3;
        // N s_99_5: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#20481:u8
        let s_100_0: bool = fn_state.gs_20481;
        // N s_100_1: branch s_100_0 b103 b101
        if s_100_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var size:i
        let s_101_0: i128 = fn_state.size;
        // D s_101_1: call __id(s_101_0)
        let s_101_1: i128 = u__id(state, tracer, s_101_0);
        // C s_101_2: const #16s : i
        let s_101_2: i128 = 16;
        // D s_101_3: cmp-eq s_101_1 s_101_2
        let s_101_3: bool = ((s_101_1) == (s_101_2));
        // D s_101_4: write-var gs#20483 <= s_101_3
        fn_state.gs_20483 = s_101_3;
        // N s_101_5: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#20483:u8
        let s_102_0: bool = fn_state.gs_20483;
        // N s_102_1: assert s_102_0
        let s_102_1: () = assert!(s_102_0);
        // D s_102_2: read-var size:i
        let s_102_2: i128 = fn_state.size;
        // D s_102_3: cast reint s_102_2 -> i64
        let s_102_3: i64 = (s_102_2 as i64);
        // D s_102_4: read-var address:u64
        let s_102_4: u64 = fn_state.address;
        // D s_102_5: read-var accdesc:struct
        let s_102_5: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_102_6: read-var aligned:u8
        let s_102_6: bool = fn_state.aligned;
        // D s_102_7: read-var ispair:u8
        let s_102_7: bool = fn_state.ispair;
        // D s_102_8: read-var value_name:bv
        let s_102_8: Bits = fn_state.value_name;
        // D s_102_9: call AArch64_MemSingle_set__1(s_102_4, s_102_3, s_102_5, s_102_6, s_102_7, s_102_8)
        let s_102_9: () = AArch64_MemSingle_set__1(
            state,
            tracer,
            s_102_4,
            s_102_3,
            s_102_5,
            s_102_6,
            s_102_7,
            s_102_8,
        );
        // N s_102_10: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #1u : u8
        let s_103_0: bool = true;
        // D s_103_1: write-var gs#20483 <= s_103_0
        fn_state.gs_20483 = s_103_0;
        // N s_103_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #1u : u8
        let s_104_0: bool = true;
        // D s_104_1: write-var gs#20481 <= s_104_0
        fn_state.gs_20481 = s_104_0;
        // N s_104_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #1u : u8
        let s_105_0: bool = true;
        // D s_105_1: write-var gs#20479 <= s_105_0
        fn_state.gs_20479 = s_105_0;
        // N s_105_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #1u : u8
        let s_106_0: bool = true;
        // D s_106_1: write-var gs#20477 <= s_106_0
        fn_state.gs_20477 = s_106_0;
        // N s_106_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #16s : i
        let s_107_0: i128 = 16;
        // D s_107_1: read-var address:u64
        let s_107_1: u64 = fn_state.address;
        // D s_107_2: read-var size:i
        let s_107_2: i128 = fn_state.size;
        // D s_107_3: call AllInAlignedQuantity(s_107_1, s_107_2, s_107_0)
        let s_107_3: bool = AllInAlignedQuantity(
            state,
            tracer,
            s_107_1,
            s_107_2,
            s_107_0,
        );
        // D s_107_4: write-var gs#20364 <= s_107_3
        fn_state.gs_20364 = s_107_3;
        // N s_107_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #0s : i
        let s_108_0: i128 = 0;
        // D s_108_1: read-var halfsize:i
        let s_108_1: i128 = fn_state.halfsize;
        // D s_108_2: cmp-gt s_108_1 s_108_0
        let s_108_2: bool = ((s_108_1) > (s_108_0));
        // N s_108_3: assert s_108_2
        let s_108_3: () = assert!(s_108_2);
        // C s_108_4: const #64s : i
        let s_108_4: i128 = 64;
        // D s_108_5: read-var value_name:bv
        let s_108_5: Bits = fn_state.value_name;
        // C s_108_6: const #1s : i64
        let s_108_6: i64 = 1;
        // C s_108_7: cast zx s_108_6 -> i
        let s_108_7: i128 = (i128::try_from(s_108_6).unwrap());
        // C s_108_8: const #63s : i
        let s_108_8: i128 = 63;
        // C s_108_9: add s_108_8 s_108_7
        let s_108_9: i128 = (s_108_8 + s_108_7);
        // D s_108_10: bit-extract s_108_5 s_108_4 s_108_9
        let s_108_10: Bits = (Bits::new(
            ((s_108_5) >> (s_108_4)).value(),
            u16::try_from(s_108_9).unwrap(),
        ));
        // D s_108_11: write-var highhalf <= s_108_10
        fn_state.highhalf = s_108_10;
        // C s_108_12: const #0s : i
        let s_108_12: i128 = 0;
        // D s_108_13: read-var value_name:bv
        let s_108_13: Bits = fn_state.value_name;
        // C s_108_14: const #1s : i64
        let s_108_14: i64 = 1;
        // C s_108_15: cast zx s_108_14 -> i
        let s_108_15: i128 = (i128::try_from(s_108_14).unwrap());
        // C s_108_16: const #63s : i
        let s_108_16: i128 = 63;
        // C s_108_17: add s_108_16 s_108_15
        let s_108_17: i128 = (s_108_16 + s_108_15);
        // D s_108_18: bit-extract s_108_13 s_108_12 s_108_17
        let s_108_18: Bits = (Bits::new(
            ((s_108_13) >> (s_108_12)).value(),
            u16::try_from(s_108_17).unwrap(),
        ));
        // D s_108_19: read-var halfsize:i
        let s_108_19: i128 = fn_state.halfsize;
        // D s_108_20: cast reint s_108_19 -> i64
        let s_108_20: i64 = (s_108_19 as i64);
        // D s_108_21: read-var address:u64
        let s_108_21: u64 = fn_state.address;
        // D s_108_22: read-var accdesc:struct
        let s_108_22: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_108_23: read-var aligned:u8
        let s_108_23: bool = fn_state.aligned;
        // D s_108_24: read-var ispair:u8
        let s_108_24: bool = fn_state.ispair;
        // D s_108_25: call AArch64_MemSingle_set__1(s_108_21, s_108_20, s_108_22, s_108_23, s_108_24, s_108_18)
        let s_108_25: () = AArch64_MemSingle_set__1(
            state,
            tracer,
            s_108_21,
            s_108_20,
            s_108_22,
            s_108_23,
            s_108_24,
            s_108_18,
        );
        // N s_108_26: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var address:u64
        let s_109_0: u64 = fn_state.address;
        // D s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 64u16);
        // D s_109_2: read-var halfsize:i
        let s_109_2: i128 = fn_state.halfsize;
        // D s_109_3: cast cvt s_109_2 -> bv
        let s_109_3: Bits = Bits::new(s_109_2 as u128, 128);
        // D s_109_4: add s_109_1 s_109_3
        let s_109_4: Bits = (s_109_1 + s_109_3);
        // D s_109_5: cast reint s_109_4 -> u64
        let s_109_5: u64 = (s_109_4.value() as u64);
        // D s_109_6: read-var halfsize:i
        let s_109_6: i128 = fn_state.halfsize;
        // D s_109_7: cast reint s_109_6 -> i64
        let s_109_7: i64 = (s_109_6 as i64);
        // D s_109_8: read-var accdesc:struct
        let s_109_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_109_9: read-var aligned:u8
        let s_109_9: bool = fn_state.aligned;
        // D s_109_10: read-var ispair:u8
        let s_109_10: bool = fn_state.ispair;
        // D s_109_11: read-var highhalf:bv
        let s_109_11: Bits = fn_state.highhalf;
        // D s_109_12: call AArch64_MemSingle_set__1(s_109_5, s_109_7, s_109_8, s_109_9, s_109_10, s_109_11)
        let s_109_12: () = AArch64_MemSingle_set__1(
            state,
            tracer,
            s_109_5,
            s_109_7,
            s_109_8,
            s_109_9,
            s_109_10,
            s_109_11,
        );
        // N s_109_13: return
        return;
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #8s : i
        let s_110_0: i128 = 8;
        // D s_110_1: read-var address:u64
        let s_110_1: u64 = fn_state.address;
        // D s_110_2: cast zx s_110_1 -> bv
        let s_110_2: Bits = Bits::new(s_110_1 as u128, 64u16);
        // D s_110_3: call IsAligned__1(s_110_2, s_110_0)
        let s_110_3: bool = IsAligned__1(state, tracer, s_110_2, s_110_0);
        // D s_110_4: write-var gs#20361 <= s_110_3
        fn_state.gs_20361 = s_110_3;
        // N s_110_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #16s : i
        let s_111_0: i128 = 16;
        // D s_111_1: read-var size:i
        let s_111_1: i128 = fn_state.size;
        // D s_111_2: cmp-eq s_111_1 s_111_0
        let s_111_2: bool = ((s_111_1) == (s_111_0));
        // D s_111_3: write-var gs#20359 <= s_111_2
        fn_state.gs_20359 = s_111_2;
        // N s_111_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var size:i
        let s_112_0: i128 = fn_state.size;
        // D s_112_1: call __id(s_112_0)
        let s_112_1: i128 = u__id(state, tracer, s_112_0);
        // C s_112_2: const #8s : i
        let s_112_2: i128 = 8;
        // D s_112_3: mul s_112_1 s_112_2
        let s_112_3: i128 = ((s_112_1) * (s_112_2));
        // C s_112_4: const #8s : i
        let s_112_4: i128 = 8;
        // D s_112_5: cmp-eq s_112_3 s_112_4
        let s_112_5: bool = ((s_112_3) == (s_112_4));
        // N s_112_6: branch s_112_5 b124 b113
        if s_112_5 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var size:i
        let s_113_0: i128 = fn_state.size;
        // D s_113_1: call __id(s_113_0)
        let s_113_1: i128 = u__id(state, tracer, s_113_0);
        // C s_113_2: const #8s : i
        let s_113_2: i128 = 8;
        // D s_113_3: mul s_113_1 s_113_2
        let s_113_3: i128 = ((s_113_1) * (s_113_2));
        // C s_113_4: const #16s : i
        let s_113_4: i128 = 16;
        // D s_113_5: cmp-eq s_113_3 s_113_4
        let s_113_5: bool = ((s_113_3) == (s_113_4));
        // N s_113_6: branch s_113_5 b123 b114
        if s_113_5 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var size:i
        let s_114_0: i128 = fn_state.size;
        // D s_114_1: call __id(s_114_0)
        let s_114_1: i128 = u__id(state, tracer, s_114_0);
        // C s_114_2: const #8s : i
        let s_114_2: i128 = 8;
        // D s_114_3: mul s_114_1 s_114_2
        let s_114_3: i128 = ((s_114_1) * (s_114_2));
        // C s_114_4: const #32s : i
        let s_114_4: i128 = 32;
        // D s_114_5: cmp-eq s_114_3 s_114_4
        let s_114_5: bool = ((s_114_3) == (s_114_4));
        // N s_114_6: branch s_114_5 b122 b115
        if s_114_5 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var size:i
        let s_115_0: i128 = fn_state.size;
        // D s_115_1: call __id(s_115_0)
        let s_115_1: i128 = u__id(state, tracer, s_115_0);
        // C s_115_2: const #8s : i
        let s_115_2: i128 = 8;
        // D s_115_3: mul s_115_1 s_115_2
        let s_115_3: i128 = ((s_115_1) * (s_115_2));
        // C s_115_4: const #64s : i
        let s_115_4: i128 = 64;
        // D s_115_5: cmp-eq s_115_3 s_115_4
        let s_115_5: bool = ((s_115_3) == (s_115_4));
        // N s_115_6: branch s_115_5 b121 b116
        if s_115_5 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var size:i
        let s_116_0: i128 = fn_state.size;
        // D s_116_1: call __id(s_116_0)
        let s_116_1: i128 = u__id(state, tracer, s_116_0);
        // C s_116_2: const #8s : i
        let s_116_2: i128 = 8;
        // D s_116_3: mul s_116_1 s_116_2
        let s_116_3: i128 = ((s_116_1) * (s_116_2));
        // C s_116_4: const #128s : i
        let s_116_4: i128 = 128;
        // D s_116_5: cmp-eq s_116_3 s_116_4
        let s_116_5: bool = ((s_116_3) == (s_116_4));
        // D s_116_6: write-var gs#20505 <= s_116_5
        fn_state.gs_20505 = s_116_5;
        // N s_116_7: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#20505:u8
        let s_117_0: bool = fn_state.gs_20505;
        // D s_117_1: write-var gs#20506 <= s_117_0
        fn_state.gs_20506 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#20506:u8
        let s_118_0: bool = fn_state.gs_20506;
        // D s_118_1: write-var gs#20507 <= s_118_0
        fn_state.gs_20507 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#20507:u8
        let s_119_0: bool = fn_state.gs_20507;
        // D s_119_1: write-var gs#20508 <= s_119_0
        fn_state.gs_20508 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#20508:u8
        let s_120_0: bool = fn_state.gs_20508;
        // N s_120_1: assert s_120_0
        let s_120_1: () = assert!(s_120_0);
        // D s_120_2: read-var value_name:bv
        let s_120_2: Bits = fn_state.value_name;
        // D s_120_3: call reverse_endianness(s_120_2)
        let s_120_3: Bits = reverse_endianness(state, tracer, s_120_2);
        // D s_120_4: write-var value_name <= s_120_3
        fn_state.value_name = s_120_3;
        // N s_120_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #1u : u8
        let s_121_0: bool = true;
        // D s_121_1: write-var gs#20505 <= s_121_0
        fn_state.gs_20505 = s_121_0;
        // N s_121_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #1u : u8
        let s_122_0: bool = true;
        // D s_122_1: write-var gs#20506 <= s_122_0
        fn_state.gs_20506 = s_122_0;
        // N s_122_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #1u : u8
        let s_123_0: bool = true;
        // D s_123_1: write-var gs#20507 <= s_123_0
        fn_state.gs_20507 = s_123_0;
        // N s_123_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #1u : u8
        let s_124_0: bool = true;
        // D s_124_1: write-var gs#20508 <= s_124_0
        fn_state.gs_20508 = s_124_0;
        // N s_124_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var accdesc:struct
        let s_125_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_125_1: call AlignmentFault(s_125_0)
        let s_125_1: ProductType1d757adad216cdef = AlignmentFault(
            state,
            tracer,
            s_125_0,
        );
        // D s_125_2: read-var address:u64
        let s_125_2: u64 = fn_state.address;
        // D s_125_3: call AArch64_Abort(s_125_2, s_125_1)
        let s_125_3: () = AArch64_Abort(state, tracer, s_125_2, s_125_1);
        // N s_125_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var accdesc:struct
        let s_126_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_126_1: read-var address:u64
        let s_126_1: u64 = fn_state.address;
        // D s_126_2: read-var size:i
        let s_126_2: i128 = fn_state.size;
        // D s_126_3: call AArch64_UnalignedAccessFaults(s_126_0, s_126_1, s_126_2)
        let s_126_3: bool = AArch64_UnalignedAccessFaults(
            state,
            tracer,
            s_126_0,
            s_126_1,
            s_126_2,
        );
        // D s_126_4: write-var gs#20357 <= s_126_3
        fn_state.gs_20357 = s_126_3;
        // N s_126_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var halfsize:i
        let s_127_0: i128 = fn_state.halfsize;
        // D s_127_1: write-var ga#15770 <= s_127_0
        fn_state.ga_15770 = s_127_0;
        // N s_127_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
