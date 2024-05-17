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
use VTTBR_EL2_read::*;
use HaveSecureEL2Ext::*;
use ELUsingAArch32::*;
use Have16bitVMID::*;
use u_get_VTTBR_Type_VMID::*;
use u_get_VTTBR_EL2_Type_VMID::*;
use u_get_VTCR_EL2_Type_VS::*;
use Zeros::*;
use EL2Enabled::*;
use common::*;
pub fn VMID_read<T: Tracer>(state: &mut State, tracer: &T, gs_17584: ()) -> u16 {
    #[derive(Default)]
    struct FunctionState {
        gs_17585: bool,
        return_value: u16,
        gs_17588: bool,
        gs_17584: (),
    }
    let fn_state = FunctionState {
        gs_17584,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call EL2Enabled(s_0_0)
        let s_0_1: bool = EL2Enabled(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b8 b1
        if s_0_1 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_1_0: const #432u : u32
        let s_1_0: u32 = 432;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #2u : u8
        let s_1_2: u8 = 2;
        // D s_1_3: cmp-lt s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) < (s_1_2));
        // N s_1_4: branch s_1_3 b7 b2
        if s_1_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#17585 <= s_2_0
        fn_state.gs_17585 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_3_0: read-var gs#17585:u8
        let s_3_0: bool = fn_state.gs_17585;
        // N s_3_1: branch s_3_0 b6 b4
        if s_3_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_4_0: const #1080u : u32
        let s_4_0: u32 = 1080;
        // D s_4_1: read-reg s_4_0:u16
        let s_4_1: u16 = {
            let value = state.read_register::<u16>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: write-var return_value <= s_4_1
        fn_state.return_value = s_4_1;
        // N s_4_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_5_0: read-var return_value:u16
        let s_5_0: u16 = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_6_0: const #16s : i
        let s_6_0: i128 = 16;
        // S s_6_1: call Zeros(s_6_0)
        let s_6_1: Bits = Zeros(state, tracer, s_6_0);
        // S s_6_2: cast reint s_6_1 -> u16
        let s_6_2: u16 = (s_6_1.value() as u16);
        // D s_6_3: write-var return_value <= s_6_2
        fn_state.return_value = s_6_2;
        // N s_6_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call HaveSecureEL2Ext(s_7_0)
        let s_7_1: bool = HaveSecureEL2Ext(state, tracer, s_7_0);
        // D s_7_2: write-var gs#17585 <= s_7_1
        fn_state.gs_17585 = s_7_1;
        // N s_7_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_8_0: const #432u : u32
        let s_8_0: u32 = 432;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call ELUsingAArch32(s_8_1)
        let s_8_2: bool = ELUsingAArch32(state, tracer, s_8_1);
        // D s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b10 b9
        if s_8_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_9_0: const #22408u : u32
        let s_9_0: u32 = 22408;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_VTTBR_Type_VMID(s_9_1)
        let s_9_2: u8 = u_get_VTTBR_Type_VMID(state, tracer, s_9_1);
        // C s_9_3: const #16s : i
        let s_9_3: i128 = 16;
        // D s_9_4: cast zx s_9_2 -> bv
        let s_9_4: Bits = Bits::new(s_9_2 as u128, 8u16);
        // D s_9_5: bits-cast zx s_9_4 -> bv length s_9_3
        let s_9_5: Bits = s_9_4.zero_extend(s_9_3);
        // D s_9_6: cast reint s_9_5 -> u16
        let s_9_6: u16 = (s_9_5.value() as u16);
        // D s_9_7: write-var return_value <= s_9_6
        fn_state.return_value = s_9_6;
        // N s_9_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call Have16bitVMID(s_10_0)
        let s_10_1: bool = Have16bitVMID(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b15 b11
        if s_10_1 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#17588 <= s_11_0
        fn_state.gs_17588 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_12_0: read-var gs#17588:u8
        let s_12_0: bool = fn_state.gs_17588;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call VTTBR_EL2_read(s_13_0)
        let s_13_1: ProductType782ac6922b48c20d = VTTBR_EL2_read(state, tracer, s_13_0);
        // S s_13_2: call _get_VTTBR_EL2_Type_VMID(s_13_1)
        let s_13_2: u16 = u_get_VTTBR_EL2_Type_VMID(state, tracer, s_13_1);
        // C s_13_3: const #0s : i
        let s_13_3: i128 = 0;
        // S s_13_4: cast zx s_13_2 -> bv
        let s_13_4: Bits = Bits::new(s_13_2 as u128, 16u16);
        // C s_13_5: const #1s : i64
        let s_13_5: i64 = 1;
        // C s_13_6: cast zx s_13_5 -> i
        let s_13_6: i128 = (i128::try_from(s_13_5).unwrap());
        // C s_13_7: const #7s : i
        let s_13_7: i128 = 7;
        // C s_13_8: add s_13_7 s_13_6
        let s_13_8: i128 = (s_13_7 + s_13_6);
        // D s_13_9: bit-extract s_13_4 s_13_3 s_13_8
        let s_13_9: Bits = (Bits::new(
            ((s_13_4) >> (s_13_3)).value(),
            u16::try_from(s_13_8).unwrap(),
        ));
        // D s_13_10: cast reint s_13_9 -> u8
        let s_13_10: u8 = (s_13_9.value() as u8);
        // C s_13_11: const #16s : i
        let s_13_11: i128 = 16;
        // D s_13_12: cast zx s_13_10 -> bv
        let s_13_12: Bits = Bits::new(s_13_10 as u128, 8u16);
        // D s_13_13: bits-cast zx s_13_12 -> bv length s_13_11
        let s_13_13: Bits = s_13_12.zero_extend(s_13_11);
        // D s_13_14: cast reint s_13_13 -> u16
        let s_13_14: u16 = (s_13_13.value() as u16);
        // D s_13_15: write-var return_value <= s_13_14
        fn_state.return_value = s_13_14;
        // N s_13_16: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call VTTBR_EL2_read(s_14_0)
        let s_14_1: ProductType782ac6922b48c20d = VTTBR_EL2_read(state, tracer, s_14_0);
        // S s_14_2: call _get_VTTBR_EL2_Type_VMID(s_14_1)
        let s_14_2: u16 = u_get_VTTBR_EL2_Type_VMID(state, tracer, s_14_1);
        // D s_14_3: write-var return_value <= s_14_2
        fn_state.return_value = s_14_2;
        // N s_14_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_15_0: const #15328u : u32
        let s_15_0: u32 = 15328;
        // D s_15_1: read-reg s_15_0:struct
        let s_15_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: call _get_VTCR_EL2_Type_VS(s_15_1)
        let s_15_2: bool = u_get_VTCR_EL2_Type_VS(state, tracer, s_15_1);
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // C s_15_4: const #1u : u8
        let s_15_4: bool = true;
        // C s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 1u16);
        // D s_15_6: cmp-eq s_15_3 s_15_5
        let s_15_6: bool = ((s_15_3) == (s_15_5));
        // D s_15_7: write-var gs#17588 <= s_15_6
        fn_state.gs_17588 = s_15_6;
        // N s_15_8: jump b12
        return block_12(state, tracer, fn_state);
    }
}
