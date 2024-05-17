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
use HaveSecureEL2Ext::*;
use HaveRME::*;
use SecureOnlyImplementation::*;
use IsSecureEL2Enabled::*;
use u_get_MDCR_EL3_Type_NSPB::*;
use u_get_MDCR_EL3_Type_NSPBE::*;
use u_get_MDCR_EL2_Type_E2PB::*;
use ConstrainUnpredictableBits::*;
use common::*;
pub fn ProfilingBufferOwner<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_3547: (),
) -> ProductTypec8897aad3eb4a29e {
    #[derive(Default)]
    struct FunctionState {
        gs_3560: bool,
        owning_ss: u32,
        gs_3561: bool,
        owning_el: u8,
        state_bits: u8,
        gs_3550: bool,
        gs_432694: ProductType690b94b58c91cec7,
        gs_3578: bool,
        gs_3579: bool,
        gs_3555: bool,
        gs_3547: (),
    }
    let fn_state = FunctionState {
        gs_3547,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_0_0: const #424u : u32
        let s_0_0: u32 = 424;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u8
        let s_0_2: u8 = 2;
        // D s_0_3: cmp-lt s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) < (s_0_2));
        // N s_0_4: branch s_0_3 b18 b1
        if s_0_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call SecureOnlyImplementation(s_1_0)
        let s_1_1: bool = SecureOnlyImplementation(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b17 b2
        if s_1_1 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_2_0: const #0u : u32
        let s_2_0: u32 = 0;
        // D s_2_1: write-var owning_ss <= s_2_0
        fn_state.owning_ss = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_4_0: const #432u : u32
        let s_4_0: u32 = 432;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // C s_4_2: const #2u : u8
        let s_4_2: u8 = 2;
        // D s_4_3: cmp-lt s_4_1 s_4_2
        let s_4_3: bool = ((s_4_1) < (s_4_2));
        // N s_4_4: branch s_4_3 b13 b5
        if s_4_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#3579 <= s_5_0
        fn_state.gs_3579 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // D s_6_0: read-var gs#3579:u8
        let s_6_0: bool = fn_state.gs_3579;
        // N s_6_1: branch s_6_0 b9 b7
        if s_6_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_7_0: const #440u : u32
        let s_7_0: u32 = 440;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: write-var owning_el <= s_7_1
        fn_state.owning_el = s_7_1;
        // N s_7_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // D s_8_0: read-var owning_ss:u32
        let s_8_0: u32 = fn_state.owning_ss;
        // D s_8_1: read-var owning_el:u8
        let s_8_1: u8 = fn_state.owning_el;
        // D s_8_2: create-product struct = ["s_8_0", "s_8_1"]
        let s_8_2: ProductTypec8897aad3eb4a29e = ProductTypec8897aad3eb4a29e {
            _0: s_8_0,
            _1: s_8_1,
        };
        // N s_8_3: return s_8_2
        return s_8_2;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_9_0: const #104880u : u32
        let s_9_0: u32 = 104880;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_MDCR_EL2_Type_E2PB(s_9_1)
        let s_9_2: u8 = u_get_MDCR_EL2_Type_E2PB(state, tracer, s_9_1);
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 2u16);
        // C s_9_4: const #0u : u8
        let s_9_4: u8 = 0;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 2u16);
        // D s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // N s_9_7: branch s_9_6 b12 b10
        if s_9_6 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_10_0: const #440u : u32
        let s_10_0: u32 = 440;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: write-var owning_el <= s_10_1
        fn_state.owning_el = s_10_1;
        // N s_10_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // N s_11_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_12_0: const #432u : u32
        let s_12_0: u32 = 432;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: write-var owning_el <= s_12_1
        fn_state.owning_el = s_12_1;
        // N s_12_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // D s_13_0: read-var owning_ss:u32
        let s_13_0: u32 = fn_state.owning_ss;
        // C s_13_1: const #3u : u32
        let s_13_1: u32 = 3;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b16 b14
        if s_13_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call IsSecureEL2Enabled(s_14_0)
        let s_14_1: bool = IsSecureEL2Enabled(state, tracer, s_14_0);
        // D s_14_2: write-var gs#3578 <= s_14_1
        fn_state.gs_3578 = s_14_1;
        // N s_14_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // D s_15_0: read-var gs#3578:u8
        let s_15_0: bool = fn_state.gs_3578;
        // D s_15_1: write-var gs#3579 <= s_15_0
        fn_state.gs_3579 = s_15_0;
        // N s_15_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#3578 <= s_16_0
        fn_state.gs_3578 = s_16_0;
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_17_0: const #3u : u32
        let s_17_0: u32 = 3;
        // D s_17_1: write-var owning_ss <= s_17_0
        fn_state.owning_ss = s_17_0;
        // N s_17_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call HaveRME(s_18_0)
        let s_18_1: bool = HaveRME(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b28 b19
        if s_18_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_19_0: const #22712u : u32
        let s_19_0: u32 = 22712;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_MDCR_EL3_Type_NSPB(s_19_1)
        let s_19_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_19_1);
        // C s_19_3: const #0u : u8
        let s_19_3: bool = false;
        // C s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 1u16);
        // D s_19_5: cast zx s_19_2 -> bv
        let s_19_5: Bits = Bits::new(s_19_2 as u128, 2u16);
        // C s_19_6: cast reint s_19_4 -> u128
        let s_19_6: u128 = (s_19_4.value() as u128);
        // D s_19_7: size-of s_19_4
        let s_19_7: u16 = s_19_4.length();
        // D s_19_8: cast reint s_19_5 -> u128
        let s_19_8: u128 = (s_19_5.value() as u128);
        // D s_19_9: size-of s_19_5
        let s_19_9: u16 = s_19_5.length();
        // D s_19_10: lsl s_19_6 s_19_9
        let s_19_10: u128 = s_19_6 << s_19_9;
        // D s_19_11: or s_19_10 s_19_8
        let s_19_11: u128 = ((s_19_10) | (s_19_8));
        // D s_19_12: add s_19_7 s_19_9
        let s_19_12: u16 = (s_19_7 + s_19_9);
        // D s_19_13: create-bits s_19_11 s_19_12
        let s_19_13: Bits = Bits::new(s_19_11, s_19_12);
        // D s_19_14: cast reint s_19_13 -> u8
        let s_19_14: u8 = (s_19_13.value() as u8);
        // D s_19_15: write-var state_bits <= s_19_14
        fn_state.state_bits = s_19_14;
        // N s_19_16: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // D s_20_0: read-var state_bits:u8
        let s_20_0: u8 = fn_state.state_bits;
        // C s_20_1: const #1s : i
        let s_20_1: i128 = 1;
        // D s_20_2: cast zx s_20_0 -> bv
        let s_20_2: Bits = Bits::new(s_20_0 as u128, 3u16);
        // C s_20_3: const #1s : i64
        let s_20_3: i64 = 1;
        // C s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // C s_20_5: const #1s : i
        let s_20_5: i128 = 1;
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
        let s_20_9: Bits = Bits::new(s_20_8 as u128, 2u16);
        // C s_20_10: const #0u : u8
        let s_20_10: u8 = 0;
        // C s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 2u16);
        // D s_20_12: cmp-eq s_20_9 s_20_11
        let s_20_12: bool = ((s_20_9) == (s_20_11));
        // D s_20_13: not s_20_12
        let s_20_13: bool = !s_20_12;
        // N s_20_14: branch s_20_13 b23 b21
        if s_20_13 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_21_0: const #3u : u32
        let s_21_0: u32 = 3;
        // D s_21_1: write-var owning_ss <= s_21_0
        fn_state.owning_ss = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // N s_22_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // D s_23_0: read-var state_bits:u8
        let s_23_0: u8 = fn_state.state_bits;
        // C s_23_1: const #1s : i
        let s_23_1: i128 = 1;
        // D s_23_2: cast zx s_23_0 -> bv
        let s_23_2: Bits = Bits::new(s_23_0 as u128, 3u16);
        // C s_23_3: const #1s : i64
        let s_23_3: i64 = 1;
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // C s_23_5: const #1s : i
        let s_23_5: i128 = 1;
        // C s_23_6: add s_23_5 s_23_4
        let s_23_6: i128 = (s_23_5 + s_23_4);
        // D s_23_7: bit-extract s_23_2 s_23_1 s_23_6
        let s_23_7: Bits = (Bits::new(
            ((s_23_2) >> (s_23_1)).value(),
            u16::try_from(s_23_6).unwrap(),
        ));
        // D s_23_8: cast reint s_23_7 -> u8
        let s_23_8: u8 = (s_23_7.value() as u8);
        // D s_23_9: cast zx s_23_8 -> bv
        let s_23_9: Bits = Bits::new(s_23_8 as u128, 2u16);
        // C s_23_10: const #1u : u8
        let s_23_10: u8 = 1;
        // C s_23_11: cast zx s_23_10 -> bv
        let s_23_11: Bits = Bits::new(s_23_10 as u128, 2u16);
        // D s_23_12: cmp-eq s_23_9 s_23_11
        let s_23_12: bool = ((s_23_9) == (s_23_11));
        // D s_23_13: not s_23_12
        let s_23_13: bool = !s_23_12;
        // N s_23_14: branch s_23_13 b25 b24
        if s_23_13 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_24_0: const #0u : u32
        let s_24_0: u32 = 0;
        // D s_24_1: write-var owning_ss <= s_24_0
        fn_state.owning_ss = s_24_0;
        // N s_24_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // D s_25_0: read-var state_bits:u8
        let s_25_0: u8 = fn_state.state_bits;
        // C s_25_1: const #1s : i
        let s_25_1: i128 = 1;
        // D s_25_2: cast zx s_25_0 -> bv
        let s_25_2: Bits = Bits::new(s_25_0 as u128, 3u16);
        // C s_25_3: const #1s : i64
        let s_25_3: i64 = 1;
        // C s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // C s_25_5: const #1s : i
        let s_25_5: i128 = 1;
        // C s_25_6: add s_25_5 s_25_4
        let s_25_6: i128 = (s_25_5 + s_25_4);
        // D s_25_7: bit-extract s_25_2 s_25_1 s_25_6
        let s_25_7: Bits = (Bits::new(
            ((s_25_2) >> (s_25_1)).value(),
            u16::try_from(s_25_6).unwrap(),
        ));
        // D s_25_8: cast reint s_25_7 -> u8
        let s_25_8: u8 = (s_25_7.value() as u8);
        // D s_25_9: cast zx s_25_8 -> bv
        let s_25_9: Bits = Bits::new(s_25_8 as u128, 2u16);
        // C s_25_10: const #3u : u8
        let s_25_10: u8 = 3;
        // C s_25_11: cast zx s_25_10 -> bv
        let s_25_11: Bits = Bits::new(s_25_10 as u128, 2u16);
        // D s_25_12: cmp-eq s_25_9 s_25_11
        let s_25_12: bool = ((s_25_9) == (s_25_11));
        // D s_25_13: not s_25_12
        let s_25_13: bool = !s_25_12;
        // N s_25_14: branch s_25_13 b27 b26
        if s_25_13 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_26_0: const #2u : u32
        let s_26_0: u32 = 2;
        // D s_26_1: write-var owning_ss <= s_26_0
        fn_state.owning_ss = s_26_0;
        // N s_26_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // N s_27_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_28_0: const #22712u : u32
        let s_28_0: u32 = 22712;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_MDCR_EL3_Type_NSPBE(s_28_1)
        let s_28_2: bool = u_get_MDCR_EL3_Type_NSPBE(state, tracer, s_28_1);
        // C s_28_3: const #22712u : u32
        let s_28_3: u32 = 22712;
        // D s_28_4: read-reg s_28_3:struct
        let s_28_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_3 as isize);
            tracer.read_register(s_28_3 as isize, value);
            value
        };
        // D s_28_5: call _get_MDCR_EL3_Type_NSPB(s_28_4)
        let s_28_5: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_28_4);
        // D s_28_6: cast zx s_28_2 -> bv
        let s_28_6: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_7: cast zx s_28_5 -> bv
        let s_28_7: Bits = Bits::new(s_28_5 as u128, 2u16);
        // D s_28_8: cast reint s_28_6 -> u128
        let s_28_8: u128 = (s_28_6.value() as u128);
        // D s_28_9: size-of s_28_6
        let s_28_9: u16 = s_28_6.length();
        // D s_28_10: cast reint s_28_7 -> u128
        let s_28_10: u128 = (s_28_7.value() as u128);
        // D s_28_11: size-of s_28_7
        let s_28_11: u16 = s_28_7.length();
        // D s_28_12: lsl s_28_8 s_28_11
        let s_28_12: u128 = s_28_8 << s_28_11;
        // D s_28_13: or s_28_12 s_28_10
        let s_28_13: u128 = ((s_28_12) | (s_28_10));
        // D s_28_14: add s_28_9 s_28_11
        let s_28_14: u16 = (s_28_9 + s_28_11);
        // D s_28_15: create-bits s_28_13 s_28_14
        let s_28_15: Bits = Bits::new(s_28_13, s_28_14);
        // D s_28_16: cast reint s_28_15 -> u8
        let s_28_16: u8 = (s_28_15.value() as u8);
        // D s_28_17: write-var state_bits <= s_28_16
        fn_state.state_bits = s_28_16;
        // D s_28_18: read-var state_bits:u8
        let s_28_18: u8 = fn_state.state_bits;
        // C s_28_19: const #1s : i
        let s_28_19: i128 = 1;
        // D s_28_20: cast zx s_28_18 -> bv
        let s_28_20: Bits = Bits::new(s_28_18 as u128, 3u16);
        // C s_28_21: const #1s : i64
        let s_28_21: i64 = 1;
        // C s_28_22: cast zx s_28_21 -> i
        let s_28_22: i128 = (i128::try_from(s_28_21).unwrap());
        // C s_28_23: const #1s : i
        let s_28_23: i128 = 1;
        // C s_28_24: add s_28_23 s_28_22
        let s_28_24: i128 = (s_28_23 + s_28_22);
        // D s_28_25: bit-extract s_28_20 s_28_19 s_28_24
        let s_28_25: Bits = (Bits::new(
            ((s_28_20) >> (s_28_19)).value(),
            u16::try_from(s_28_24).unwrap(),
        ));
        // D s_28_26: cast reint s_28_25 -> u8
        let s_28_26: u8 = (s_28_25.value() as u8);
        // D s_28_27: cast zx s_28_26 -> bv
        let s_28_27: Bits = Bits::new(s_28_26 as u128, 2u16);
        // C s_28_28: const #2u : u8
        let s_28_28: u8 = 2;
        // C s_28_29: cast zx s_28_28 -> bv
        let s_28_29: Bits = Bits::new(s_28_28 as u128, 2u16);
        // D s_28_30: cmp-eq s_28_27 s_28_29
        let s_28_30: bool = ((s_28_27) == (s_28_29));
        // D s_28_31: not s_28_30
        let s_28_31: bool = !s_28_30;
        // N s_28_32: branch s_28_31 b43 b29
        if s_28_31 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#3550 <= s_29_0
        fn_state.gs_3550 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // D s_30_0: read-var gs#3550:u8
        let s_30_0: bool = fn_state.gs_3550;
        // N s_30_1: branch s_30_0 b42 b31
        if s_30_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call HaveSecureEL2Ext(s_31_0)
        let s_31_1: bool = HaveSecureEL2Ext(state, tracer, s_31_0);
        // S s_31_2: not s_31_1
        let s_31_2: bool = !s_31_1;
        // N s_31_3: branch s_31_2 b38 b32
        if s_31_2 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#3560 <= s_32_0
        fn_state.gs_3560 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // D s_33_0: read-var gs#3560:u8
        let s_33_0: bool = fn_state.gs_3560;
        // D s_33_1: write-var gs#3561 <= s_33_0
        fn_state.gs_3561 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // D s_34_0: read-var gs#3561:u8
        let s_34_0: bool = fn_state.gs_3561;
        // N s_34_1: branch s_34_0 b37 b35
        if s_34_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // N s_35_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // N s_36_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_37_0: const #3s : i
        let s_37_0: i128 = 3;
        // C s_37_1: const #64u : u32
        let s_37_1: u32 = 64;
        // S s_37_2: call ConstrainUnpredictableBits(s_37_1, s_37_0)
        let s_37_2: ProductType690b94b58c91cec7 = ConstrainUnpredictableBits(
            state,
            tracer,
            s_37_1,
            s_37_0,
        );
        // D s_37_3: write-var gs#432694 <= s_37_2
        fn_state.gs_432694 = s_37_2;
        // D s_37_4: read-var gs#432694.1:struct
        let s_37_4: Bits = fn_state.gs_432694._1;
        // D s_37_5: cast reint s_37_4 -> u8
        let s_37_5: u8 = (s_37_4.value() as u8);
        // D s_37_6: write-var state_bits <= s_37_5
        fn_state.state_bits = s_37_5;
        // N s_37_7: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // D s_38_0: read-var state_bits:u8
        let s_38_0: u8 = fn_state.state_bits;
        // C s_38_1: const #1s : i
        let s_38_1: i128 = 1;
        // D s_38_2: cast zx s_38_0 -> bv
        let s_38_2: Bits = Bits::new(s_38_0 as u128, 3u16);
        // C s_38_3: const #1s : i64
        let s_38_3: i64 = 1;
        // C s_38_4: cast zx s_38_3 -> i
        let s_38_4: i128 = (i128::try_from(s_38_3).unwrap());
        // C s_38_5: const #1s : i
        let s_38_5: i128 = 1;
        // C s_38_6: add s_38_5 s_38_4
        let s_38_6: i128 = (s_38_5 + s_38_4);
        // D s_38_7: bit-extract s_38_2 s_38_1 s_38_6
        let s_38_7: Bits = (Bits::new(
            ((s_38_2) >> (s_38_1)).value(),
            u16::try_from(s_38_6).unwrap(),
        ));
        // D s_38_8: cast reint s_38_7 -> u8
        let s_38_8: u8 = (s_38_7.value() as u8);
        // D s_38_9: cast zx s_38_8 -> bv
        let s_38_9: Bits = Bits::new(s_38_8 as u128, 2u16);
        // C s_38_10: const #0u : u8
        let s_38_10: u8 = 0;
        // C s_38_11: cast zx s_38_10 -> bv
        let s_38_11: Bits = Bits::new(s_38_10 as u128, 2u16);
        // D s_38_12: cmp-eq s_38_9 s_38_11
        let s_38_12: bool = ((s_38_9) == (s_38_11));
        // D s_38_13: not s_38_12
        let s_38_13: bool = !s_38_12;
        // N s_38_14: branch s_38_13 b41 b39
        if s_38_13 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#3555 <= s_39_0
        fn_state.gs_3555 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // D s_40_0: read-var gs#3555:u8
        let s_40_0: bool = fn_state.gs_3555;
        // D s_40_1: write-var gs#3560 <= s_40_0
        fn_state.gs_3560 = s_40_0;
        // N s_40_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#3555 <= s_41_0
        fn_state.gs_3555 = s_41_0;
        // N s_41_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#3561 <= s_42_0
        fn_state.gs_3561 = s_42_0;
        // N s_42_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec8897aad3eb4a29e {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#3550 <= s_43_0
        fn_state.gs_3550 = s_43_0;
        // N s_43_2: jump b30
        return block_30(state, tracer, fn_state);
    }
}
