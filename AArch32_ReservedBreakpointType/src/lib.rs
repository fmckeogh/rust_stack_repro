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
use ConstrainUnpredictableBits::*;
use u__UNKNOWN_bits::*;
use HaveV82Debug::*;
use HaveVirtHostExt::*;
use HaltOnBreakpointOrWatchpoint::*;
use NumContextAwareBreakpointsImplemented::*;
use NumBreakpointsImplemented::*;
use common::*;
pub fn AArch32_ReservedBreakpointType<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i128,
    bt_in: u8,
) -> ProductType54c7e1b9151093d0 {
    #[derive(Default)]
    struct FunctionState {
        gs_29717: bool,
        gs_29711: bool,
        gs_29731: bool,
        return_value: ProductType54c7e1b9151093d0,
        reserved: bool,
        gs_29741: bool,
        gs_454742: ProductType690b94b58c91cec7,
        context_aware: bool,
        gs_29716: bool,
        gs_29747: bool,
        gs_29726: bool,
        bt: u8,
        gs_29740: bool,
        gs_29725: bool,
        b__1: u8,
        c: u32,
        gs_29732: bool,
        gs_29722: bool,
        n: i128,
        bt_in: u8,
    }
    let fn_state = FunctionState {
        n,
        bt_in,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_0_0: read-var bt_in:u8
        let s_0_0: u8 = fn_state.bt_in;
        // D s_0_1: write-var bt <= s_0_0
        fn_state.bt = s_0_0;
        // C s_0_2: const #0u : u8
        let s_0_2: bool = false;
        // D s_0_3: write-var reserved <= s_0_2
        fn_state.reserved = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call NumBreakpointsImplemented(s_0_4)
        let s_0_5: i128 = NumBreakpointsImplemented(state, tracer, s_0_4);
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call NumContextAwareBreakpointsImplemented(s_0_6)
        let s_0_7: i128 = NumContextAwareBreakpointsImplemented(state, tracer, s_0_6);
        // S s_0_8: sub s_0_5 s_0_7
        let s_0_8: i128 = ((s_0_5) - (s_0_7));
        // D s_0_9: read-var n:i
        let s_0_9: i128 = fn_state.n;
        // D s_0_10: cmp-ge s_0_9 s_0_8
        let s_0_10: bool = ((s_0_9) >= (s_0_8));
        // D s_0_11: write-var context_aware <= s_0_10
        fn_state.context_aware = s_0_10;
        // D s_0_12: read-var bt:u8
        let s_0_12: u8 = fn_state.bt;
        // C s_0_13: const #1s : i
        let s_0_13: i128 = 1;
        // D s_0_14: cast zx s_0_12 -> bv
        let s_0_14: Bits = Bits::new(s_0_12 as u128, 4u16);
        // C s_0_15: const #1s : i64
        let s_0_15: i64 = 1;
        // C s_0_16: cast zx s_0_15 -> i
        let s_0_16: i128 = (i128::try_from(s_0_15).unwrap());
        // C s_0_17: const #2s : i
        let s_0_17: i128 = 2;
        // C s_0_18: add s_0_17 s_0_16
        let s_0_18: i128 = (s_0_17 + s_0_16);
        // D s_0_19: bit-extract s_0_14 s_0_13 s_0_18
        let s_0_19: Bits = (Bits::new(
            ((s_0_14) >> (s_0_13)).value(),
            u16::try_from(s_0_18).unwrap(),
        ));
        // D s_0_20: cast reint s_0_19 -> u8
        let s_0_20: u8 = (s_0_19.value() as u8);
        // D s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 3u16);
        // C s_0_22: const #2u : u8
        let s_0_22: u8 = 2;
        // C s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 3u16);
        // D s_0_24: cmp-eq s_0_21 s_0_23
        let s_0_24: bool = ((s_0_21) == (s_0_23));
        // D s_0_25: not s_0_24
        let s_0_25: bool = !s_0_24;
        // N s_0_26: branch s_0_25 b53 b1
        if s_0_25 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_1_0: const #1u : u8
        let s_1_0: bool = true;
        // D s_1_1: write-var gs#29711 <= s_1_0
        fn_state.gs_29711 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_2_0: read-var gs#29711:u8
        let s_2_0: bool = fn_state.gs_29711;
        // N s_2_1: branch s_2_0 b52 b3
        if s_2_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#29716 <= s_3_0
        fn_state.gs_29716 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_4_0: read-var gs#29716:u8
        let s_4_0: bool = fn_state.gs_29716;
        // N s_4_1: branch s_4_0 b51 b5
        if s_4_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_6_0: read-var bt:u8
        let s_6_0: u8 = fn_state.bt;
        // D s_6_1: write-var b__1 <= s_6_0
        fn_state.b__1 = s_6_0;
        // C s_6_2: const #3s : i
        let s_6_2: i128 = 3;
        // D s_6_3: read-var b__1:u8
        let s_6_3: u8 = fn_state.b__1;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 4u16);
        // C s_6_5: const #1s : i64
        let s_6_5: i64 = 1;
        // C s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // C s_6_7: const #0s : i
        let s_6_7: i128 = 0;
        // C s_6_8: add s_6_7 s_6_6
        let s_6_8: i128 = (s_6_7 + s_6_6);
        // D s_6_9: bit-extract s_6_4 s_6_2 s_6_8
        let s_6_9: Bits = (Bits::new(
            ((s_6_4) >> (s_6_2)).value(),
            u16::try_from(s_6_8).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u8
        let s_6_10: bool = ((s_6_9.value()) != 0);
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 1u16);
        // C s_6_12: const #0u : u8
        let s_6_12: bool = false;
        // C s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 1u16);
        // D s_6_14: cmp-eq s_6_11 s_6_13
        let s_6_14: bool = ((s_6_11) == (s_6_13));
        // N s_6_15: branch s_6_14 b50 b7
        if s_6_14 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#29722 <= s_7_0
        fn_state.gs_29722 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_8_0: read-var gs#29722:u8
        let s_8_0: bool = fn_state.gs_29722;
        // D s_8_1: not s_8_0
        let s_8_1: bool = !s_8_0;
        // N s_8_2: branch s_8_1 b49 b9
        if s_8_1 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#29717 <= s_9_0
        fn_state.gs_29717 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_10_0: read-var gs#29717:u8
        let s_10_0: bool = fn_state.gs_29717;
        // D s_10_1: not s_10_0
        let s_10_1: bool = !s_10_0;
        // N s_10_2: branch s_10_1 b48 b11
        if s_10_1 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#29725 <= s_11_0
        fn_state.gs_29725 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_12_0: read-var gs#29725:u8
        let s_12_0: bool = fn_state.gs_29725;
        // N s_12_1: branch s_12_0 b47 b13
        if s_12_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_14_0: read-var bt:u8
        let s_14_0: u8 = fn_state.bt;
        // C s_14_1: const #3s : i
        let s_14_1: i128 = 3;
        // D s_14_2: cast zx s_14_0 -> bv
        let s_14_2: Bits = Bits::new(s_14_0 as u128, 4u16);
        // C s_14_3: const #1s : i64
        let s_14_3: i64 = 1;
        // C s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // C s_14_5: const #0s : i
        let s_14_5: i128 = 0;
        // C s_14_6: add s_14_5 s_14_4
        let s_14_6: i128 = (s_14_5 + s_14_4);
        // D s_14_7: bit-extract s_14_2 s_14_1 s_14_6
        let s_14_7: Bits = (Bits::new(
            ((s_14_2) >> (s_14_1)).value(),
            u16::try_from(s_14_6).unwrap(),
        ));
        // D s_14_8: cast reint s_14_7 -> u8
        let s_14_8: bool = ((s_14_7.value()) != 0);
        // D s_14_9: cast zx s_14_8 -> bv
        let s_14_9: Bits = Bits::new(s_14_8 as u128, 1u16);
        // C s_14_10: const #1u : u8
        let s_14_10: bool = true;
        // C s_14_11: cast zx s_14_10 -> bv
        let s_14_11: Bits = Bits::new(s_14_10 as u128, 1u16);
        // D s_14_12: cmp-eq s_14_9 s_14_11
        let s_14_12: bool = ((s_14_9) == (s_14_11));
        // D s_14_13: not s_14_12
        let s_14_13: bool = !s_14_12;
        // N s_14_14: branch s_14_13 b46 b15
        if s_14_13 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#29726 <= s_15_0
        fn_state.gs_29726 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_16_0: read-var gs#29726:u8
        let s_16_0: bool = fn_state.gs_29726;
        // N s_16_1: branch s_16_0 b45 b17
        if s_16_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#29731 <= s_17_0
        fn_state.gs_29731 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_18_0: read-var gs#29731:u8
        let s_18_0: bool = fn_state.gs_29731;
        // N s_18_1: branch s_18_0 b44 b19
        if s_18_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_20_0: read-var bt:u8
        let s_20_0: u8 = fn_state.bt;
        // C s_20_1: const #1s : i
        let s_20_1: i128 = 1;
        // D s_20_2: cast zx s_20_0 -> bv
        let s_20_2: Bits = Bits::new(s_20_0 as u128, 4u16);
        // C s_20_3: const #1s : i64
        let s_20_3: i64 = 1;
        // C s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // C s_20_5: const #2s : i
        let s_20_5: i128 = 2;
        // C s_20_6: add s_20_5 s_20_4
        let s_20_6: i128 = (s_20_5 + s_20_4);
        // D s_20_7: bit-extract s_20_2 s_20_1 s_20_6
        let s_20_7: Bits = (Bits::new(
            ((s_20_2) >> (s_20_1)).value(),
            u16::try_from(s_20_6).unwrap(),
        ));
        // D s_20_8: cast reint s_20_7 -> u8
        let s_20_8: u8 = (s_20_7.value() as u8);
        // D s_20_9: cast zx s_20_8 -> bv
        let s_20_9: Bits = Bits::new(s_20_8 as u128, 3u16);
        // C s_20_10: const #3u : u8
        let s_20_10: u8 = 3;
        // C s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 3u16);
        // D s_20_12: cmp-eq s_20_9 s_20_11
        let s_20_12: bool = ((s_20_9) == (s_20_11));
        // D s_20_13: not s_20_12
        let s_20_13: bool = !s_20_12;
        // N s_20_14: branch s_20_13 b41 b21
        if s_20_13 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#29732 <= s_21_0
        fn_state.gs_29732 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_22_0: read-var gs#29732:u8
        let s_22_0: bool = fn_state.gs_29732;
        // N s_22_1: branch s_22_0 b40 b23
        if s_22_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#29740 <= s_23_0
        fn_state.gs_29740 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_24_0: read-var gs#29740:u8
        let s_24_0: bool = fn_state.gs_29740;
        // N s_24_1: branch s_24_0 b39 b25
        if s_24_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#29741 <= s_25_0
        fn_state.gs_29741 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_26_0: read-var gs#29741:u8
        let s_26_0: bool = fn_state.gs_29741;
        // N s_26_1: branch s_26_0 b38 b27
        if s_26_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_28_0: read-var reserved:u8
        let s_28_0: bool = fn_state.reserved;
        // N s_28_1: branch s_28_0 b32 b29
        if s_28_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_30_0: const #0u : u32
        let s_30_0: u32 = 0;
        // D s_30_1: read-var bt:u8
        let s_30_1: u8 = fn_state.bt;
        // D s_30_2: create-product struct = ["s_30_0", "s_30_1"]
        let s_30_2: ProductType54c7e1b9151093d0 = ProductType54c7e1b9151093d0 {
            _0: s_30_0,
            _1: s_30_1,
        };
        // D s_30_3: write-var return_value <= s_30_2
        fn_state.return_value = s_30_2;
        // N s_30_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_31_0: read-var return_value:struct
        let s_31_0: ProductType54c7e1b9151093d0 = fn_state.return_value;
        // N s_31_1: return s_31_0
        return s_31_0;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_32_0: const #4s : i
        let s_32_0: i128 = 4;
        // C s_32_1: const #33u : u32
        let s_32_1: u32 = 33;
        // S s_32_2: call ConstrainUnpredictableBits(s_32_1, s_32_0)
        let s_32_2: ProductType690b94b58c91cec7 = ConstrainUnpredictableBits(
            state,
            tracer,
            s_32_1,
            s_32_0,
        );
        // D s_32_3: write-var gs#454742 <= s_32_2
        fn_state.gs_454742 = s_32_2;
        // D s_32_4: read-var gs#454742.0:struct
        let s_32_4: u32 = fn_state.gs_454742._0;
        // D s_32_5: read-var gs#454742.1:struct
        let s_32_5: Bits = fn_state.gs_454742._1;
        // D s_32_6: cast reint s_32_5 -> u8
        let s_32_6: u8 = (s_32_5.value() as u8);
        // D s_32_7: write-var c <= s_32_4
        fn_state.c = s_32_4;
        // D s_32_8: write-var bt <= s_32_6
        fn_state.bt = s_32_6;
        // D s_32_9: read-var c:u32
        let s_32_9: u32 = fn_state.c;
        // C s_32_10: const #7u : u32
        let s_32_10: u32 = 7;
        // D s_32_11: cmp-eq s_32_9 s_32_10
        let s_32_11: bool = ((s_32_9) == (s_32_10));
        // N s_32_12: branch s_32_11 b37 b33
        if s_32_11 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_33_0: read-var c:u32
        let s_33_0: u32 = fn_state.c;
        // C s_33_1: const #1u : u32
        let s_33_1: u32 = 1;
        // D s_33_2: cmp-eq s_33_0 s_33_1
        let s_33_2: bool = ((s_33_0) == (s_33_1));
        // D s_33_3: write-var gs#29747 <= s_33_2
        fn_state.gs_29747 = s_33_2;
        // N s_33_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_34_0: read-var gs#29747:u8
        let s_34_0: bool = fn_state.gs_29747;
        // N s_34_1: assert s_34_0
        let s_34_1: () = assert!(s_34_0);
        // D s_34_2: read-var c:u32
        let s_34_2: u32 = fn_state.c;
        // C s_34_3: const #7u : u32
        let s_34_3: u32 = 7;
        // D s_34_4: cmp-eq s_34_2 s_34_3
        let s_34_4: bool = ((s_34_2) == (s_34_3));
        // N s_34_5: branch s_34_4 b36 b35
        if s_34_4 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // N s_35_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_36_0: const #4s : i64
        let s_36_0: i64 = 4;
        // C s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // S s_36_2: call __UNKNOWN_bits(s_36_1)
        let s_36_2: Bits = u__UNKNOWN_bits(state, tracer, s_36_1);
        // S s_36_3: cast reint s_36_2 -> u8
        let s_36_3: u8 = (s_36_2.value() as u8);
        // D s_36_4: read-var c:u32
        let s_36_4: u32 = fn_state.c;
        // D s_36_5: create-product struct = ["s_36_4", "s_36_3"]
        let s_36_5: ProductType54c7e1b9151093d0 = ProductType54c7e1b9151093d0 {
            _0: s_36_4,
            _1: s_36_3,
        };
        // D s_36_6: write-var return_value <= s_36_5
        fn_state.return_value = s_36_5;
        // N s_36_7: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#29747 <= s_37_0
        fn_state.gs_29747 = s_37_0;
        // N s_37_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var reserved <= s_38_0
        fn_state.reserved = s_38_0;
        // N s_38_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call HaveV82Debug(s_39_0)
        let s_39_1: bool = HaveV82Debug(state, tracer, s_39_0);
        // S s_39_2: not s_39_1
        let s_39_2: bool = !s_39_1;
        // D s_39_3: write-var gs#29741 <= s_39_2
        fn_state.gs_29741 = s_39_2;
        // N s_39_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call HaveVirtHostExt(s_40_0)
        let s_40_1: bool = HaveVirtHostExt(state, tracer, s_40_0);
        // S s_40_2: not s_40_1
        let s_40_2: bool = !s_40_1;
        // D s_40_3: write-var gs#29740 <= s_40_2
        fn_state.gs_29740 = s_40_2;
        // N s_40_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_41_0: read-var bt:u8
        let s_41_0: u8 = fn_state.bt;
        // C s_41_1: const #2s : i
        let s_41_1: i128 = 2;
        // D s_41_2: cast zx s_41_0 -> bv
        let s_41_2: Bits = Bits::new(s_41_0 as u128, 4u16);
        // C s_41_3: const #1s : i64
        let s_41_3: i64 = 1;
        // C s_41_4: cast zx s_41_3 -> i
        let s_41_4: i128 = (i128::try_from(s_41_3).unwrap());
        // C s_41_5: const #1s : i
        let s_41_5: i128 = 1;
        // C s_41_6: add s_41_5 s_41_4
        let s_41_6: i128 = (s_41_5 + s_41_4);
        // D s_41_7: bit-extract s_41_2 s_41_1 s_41_6
        let s_41_7: Bits = (Bits::new(
            ((s_41_2) >> (s_41_1)).value(),
            u16::try_from(s_41_6).unwrap(),
        ));
        // D s_41_8: cast reint s_41_7 -> u8
        let s_41_8: u8 = (s_41_7.value() as u8);
        // D s_41_9: cast zx s_41_8 -> bv
        let s_41_9: Bits = Bits::new(s_41_8 as u128, 2u16);
        // C s_41_10: const #3u : u8
        let s_41_10: u8 = 3;
        // C s_41_11: cast zx s_41_10 -> bv
        let s_41_11: Bits = Bits::new(s_41_10 as u128, 2u16);
        // D s_41_12: cmp-eq s_41_9 s_41_11
        let s_41_12: bool = ((s_41_9) == (s_41_11));
        // D s_41_13: not s_41_12
        let s_41_13: bool = !s_41_12;
        // N s_41_14: branch s_41_13 b43 b42
        if s_41_13 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#29732 <= s_42_0
        fn_state.gs_29732 = s_42_0;
        // N s_42_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#29732 <= s_43_0
        fn_state.gs_29732 = s_43_0;
        // N s_43_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var reserved <= s_44_0
        fn_state.reserved = s_44_0;
        // N s_44_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_45_0: const #432u : u32
        let s_45_0: u32 = 432;
        // D s_45_1: read-reg s_45_0:u8
        let s_45_1: u8 = {
            let value = state.read_register::<u8>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // C s_45_2: const #2u : u8
        let s_45_2: u8 = 2;
        // D s_45_3: cmp-lt s_45_1 s_45_2
        let s_45_3: bool = ((s_45_1) < (s_45_2));
        // D s_45_4: not s_45_3
        let s_45_4: bool = !s_45_3;
        // D s_45_5: write-var gs#29731 <= s_45_4
        fn_state.gs_29731 = s_45_4;
        // N s_45_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#29726 <= s_46_0
        fn_state.gs_29726 = s_46_0;
        // N s_46_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var reserved <= s_47_0
        fn_state.reserved = s_47_0;
        // N s_47_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // D s_48_0: read-var context_aware:u8
        let s_48_0: bool = fn_state.context_aware;
        // D s_48_1: not s_48_0
        let s_48_1: bool = !s_48_0;
        // D s_48_2: write-var gs#29725 <= s_48_1
        fn_state.gs_29725 = s_48_1;
        // N s_48_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#29717 <= s_49_0
        fn_state.gs_29717 = s_49_0;
        // N s_49_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_50_0: const #1s : i
        let s_50_0: i128 = 1;
        // D s_50_1: read-var b__1:u8
        let s_50_1: u8 = fn_state.b__1;
        // D s_50_2: cast zx s_50_1 -> bv
        let s_50_2: Bits = Bits::new(s_50_1 as u128, 4u16);
        // C s_50_3: const #1s : i64
        let s_50_3: i64 = 1;
        // C s_50_4: cast zx s_50_3 -> i
        let s_50_4: i128 = (i128::try_from(s_50_3).unwrap());
        // C s_50_5: const #0s : i
        let s_50_5: i128 = 0;
        // C s_50_6: add s_50_5 s_50_4
        let s_50_6: i128 = (s_50_5 + s_50_4);
        // D s_50_7: bit-extract s_50_2 s_50_0 s_50_6
        let s_50_7: Bits = (Bits::new(
            ((s_50_2) >> (s_50_0)).value(),
            u16::try_from(s_50_6).unwrap(),
        ));
        // D s_50_8: cast reint s_50_7 -> u8
        let s_50_8: bool = ((s_50_7.value()) != 0);
        // D s_50_9: cast zx s_50_8 -> bv
        let s_50_9: Bits = Bits::new(s_50_8 as u128, 1u16);
        // C s_50_10: const #0u : u8
        let s_50_10: bool = false;
        // C s_50_11: cast zx s_50_10 -> bv
        let s_50_11: Bits = Bits::new(s_50_10 as u128, 1u16);
        // D s_50_12: cmp-eq s_50_9 s_50_11
        let s_50_12: bool = ((s_50_9) == (s_50_11));
        // D s_50_13: write-var gs#29722 <= s_50_12
        fn_state.gs_29722 = s_50_12;
        // N s_50_14: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_51_0: const #1u : u8
        let s_51_0: bool = true;
        // D s_51_1: write-var reserved <= s_51_0
        fn_state.reserved = s_51_0;
        // N s_51_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call HaltOnBreakpointOrWatchpoint(s_52_0)
        let s_52_1: bool = HaltOnBreakpointOrWatchpoint(state, tracer, s_52_0);
        // D s_52_2: write-var gs#29716 <= s_52_1
        fn_state.gs_29716 = s_52_1;
        // N s_52_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType54c7e1b9151093d0 {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#29711 <= s_53_0
        fn_state.gs_29711 = s_53_0;
        // N s_53_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
