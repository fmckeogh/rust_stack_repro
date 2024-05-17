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
use u_get_FPCR_Type_DZE::*;
use u_get_FPCR_Type_Stride::*;
use u_get_FPCR_Type_IDE::*;
use Bit::*;
use u__IMPDEF_boolean::*;
use u_get_FPCR_Type_IOE::*;
use u_get_FPCR_Type_OFE::*;
use u_get_FPCR_Type_UFE::*;
use u_get_FPCR_Type_IXE::*;
use u_get_FPCR_Type_length::*;
use common::*;
pub fn u__set_FPCR<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        value_name: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #12920u : u32
        let s_0_2: u32 = 12920;
        // D s_0_3: read-reg s_0_2:struct
        let s_0_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #12920u : u32
        let s_0_4: u32 = 12920;
        // N s_0_5: write-reg s_0_4 <= s_0_3
        let s_0_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_4 as isize, s_0_3);
            tracer.write_register(s_0_4 as isize, s_0_3);
        };
        // C s_0_6: const #12920u : u32
        let s_0_6: u32 = 12920;
        // D s_0_7: read-reg s_0_6:struct
        let s_0_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // C s_0_8: const #12920u : u32
        let s_0_8: u32 = 12920;
        // N s_0_9: write-reg s_0_8 <= s_0_7
        let s_0_9: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_8 as isize, s_0_7);
            tracer.write_register(s_0_8 as isize, s_0_7);
        };
        // D s_0_10: read-var r.0:struct
        let s_0_10: u64 = fn_state.r._0;
        // C s_0_11: const #19s : i
        let s_0_11: i128 = 19;
        // C s_0_12: const #1s : i
        let s_0_12: i128 = 1;
        // D s_0_13: cast zx s_0_10 -> bv
        let s_0_13: Bits = Bits::new(s_0_10 as u128, 64u16);
        // D s_0_14: bit-extract s_0_13 s_0_11 s_0_12
        let s_0_14: Bits = (Bits::new(
            ((s_0_13) >> (s_0_11)).value(),
            u16::try_from(s_0_12).unwrap(),
        ));
        // D s_0_15: cast reint s_0_14 -> u8
        let s_0_15: bool = ((s_0_14.value()) != 0);
        // D s_0_16: call Bit(s_0_15)
        let s_0_16: bool = Bit(state, tracer, s_0_15);
        // C s_0_17: const #12920u : u32
        let s_0_17: u32 = 12920;
        // D s_0_18: read-reg s_0_17:struct
        let s_0_18: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // C s_0_19: const #12920u : u32
        let s_0_19: u32 = 12920;
        // N s_0_20: write-reg s_0_19 <= s_0_18
        let s_0_20: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize, s_0_18);
            tracer.write_register(s_0_19 as isize, s_0_18);
        };
        // C s_0_21: const #12920u : u32
        let s_0_21: u32 = 12920;
        // D s_0_22: read-reg s_0_21:struct
        let s_0_22: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // C s_0_23: const #12920u : u32
        let s_0_23: u32 = 12920;
        // N s_0_24: write-reg s_0_23 <= s_0_22
        let s_0_24: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize, s_0_22);
            tracer.write_register(s_0_23 as isize, s_0_22);
        };
        // C s_0_25: const #"IMPLEMENTED_FPSCR.LEN,STRIDE as RAZ" : str
        let s_0_25: &'static str = "IMPLEMENTED_FPSCR.LEN,STRIDE as RAZ";
        // S s_0_26: call __IMPDEF_boolean(s_0_25)
        let s_0_26: bool = u__IMPDEF_boolean(state, tracer, s_0_25);
        // S s_0_27: not s_0_26
        let s_0_27: bool = !s_0_26;
        // N s_0_28: branch s_0_27 b23 b1
        if s_0_27 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #"IMPLEMENTED_FPSCR.LEN,STRIDE as RAZ" : str
        let s_2_0: &'static str = "IMPLEMENTED_FPSCR.LEN,STRIDE as RAZ";
        // S s_2_1: call __IMPDEF_boolean(s_2_0)
        let s_2_1: bool = u__IMPDEF_boolean(state, tracer, s_2_0);
        // S s_2_2: not s_2_1
        let s_2_2: bool = !s_2_1;
        // N s_2_3: branch s_2_2 b22 b3
        if s_2_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #"IMPLEMENTED_trapping of Input Denormal floating-point exceptions" : str
        let s_4_0: &'static str = "IMPLEMENTED_trapping of Input Denormal floating-point exceptions";
        // S s_4_1: call __IMPDEF_boolean(s_4_0)
        let s_4_1: bool = u__IMPDEF_boolean(state, tracer, s_4_0);
        // S s_4_2: not s_4_1
        let s_4_2: bool = !s_4_1;
        // S s_4_3: not s_4_2
        let s_4_3: bool = !s_4_2;
        // N s_4_4: branch s_4_3 b21 b5
        if s_4_3 {
            return block_21(state, tracer, fn_state);
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
        // C s_6_0: const #"IMPLEMENTED_trapping of Inexact floating-point exceptions" : str
        let s_6_0: &'static str = "IMPLEMENTED_trapping of Inexact floating-point exceptions";
        // S s_6_1: call __IMPDEF_boolean(s_6_0)
        let s_6_1: bool = u__IMPDEF_boolean(state, tracer, s_6_0);
        // S s_6_2: not s_6_1
        let s_6_2: bool = !s_6_1;
        // S s_6_3: not s_6_2
        let s_6_3: bool = !s_6_2;
        // N s_6_4: branch s_6_3 b20 b7
        if s_6_3 {
            return block_20(state, tracer, fn_state);
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
        // C s_8_0: const #"IMPLEMENTED_trapping of Underflow floating-point exceptions" : str
        let s_8_0: &'static str = "IMPLEMENTED_trapping of Underflow floating-point exceptions";
        // S s_8_1: call __IMPDEF_boolean(s_8_0)
        let s_8_1: bool = u__IMPDEF_boolean(state, tracer, s_8_0);
        // S s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // S s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b19 b9
        if s_8_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #"IMPLEMENTED_trapping of Overflow floating-point exceptions" : str
        let s_10_0: &'static str = "IMPLEMENTED_trapping of Overflow floating-point exceptions";
        // S s_10_1: call __IMPDEF_boolean(s_10_0)
        let s_10_1: bool = u__IMPDEF_boolean(state, tracer, s_10_0);
        // S s_10_2: not s_10_1
        let s_10_2: bool = !s_10_1;
        // S s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b18 b11
        if s_10_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #"IMPLEMENTED_trapping of Divide by Zero floating-point exceptions" : str
        let s_12_0: &'static str = "IMPLEMENTED_trapping of Divide by Zero floating-point exceptions";
        // S s_12_1: call __IMPDEF_boolean(s_12_0)
        let s_12_1: bool = u__IMPDEF_boolean(state, tracer, s_12_0);
        // S s_12_2: not s_12_1
        let s_12_2: bool = !s_12_1;
        // S s_12_3: not s_12_2
        let s_12_3: bool = !s_12_2;
        // N s_12_4: branch s_12_3 b17 b13
        if s_12_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #"IMPLEMENTED_trapping of Invalid Operation floating-point exceptions" : str
        let s_14_0: &'static str = "IMPLEMENTED_trapping of Invalid Operation floating-point exceptions";
        // S s_14_1: call __IMPDEF_boolean(s_14_0)
        let s_14_1: bool = u__IMPDEF_boolean(state, tracer, s_14_0);
        // S s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // S s_14_3: not s_14_2
        let s_14_3: bool = !s_14_2;
        // N s_14_4: branch s_14_3 b16 b15
        if s_14_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var r:struct
        let s_16_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_16_1: call _get_FPCR_Type_IOE(s_16_0)
        let s_16_1: bool = u_get_FPCR_Type_IOE(state, tracer, s_16_0);
        // C s_16_2: const #12920u : u32
        let s_16_2: u32 = 12920;
        // D s_16_3: read-reg s_16_2:struct
        let s_16_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_2 as isize);
            tracer.read_register(s_16_2 as isize, value);
            value
        };
        // C s_16_4: const #12920u : u32
        let s_16_4: u32 = 12920;
        // N s_16_5: write-reg s_16_4 <= s_16_3
        let s_16_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_16_4 as isize, s_16_3);
            tracer.write_register(s_16_4 as isize, s_16_3);
        };
        // N s_16_6: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var r:struct
        let s_17_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_17_1: call _get_FPCR_Type_DZE(s_17_0)
        let s_17_1: bool = u_get_FPCR_Type_DZE(state, tracer, s_17_0);
        // C s_17_2: const #12920u : u32
        let s_17_2: u32 = 12920;
        // D s_17_3: read-reg s_17_2:struct
        let s_17_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_2 as isize);
            tracer.read_register(s_17_2 as isize, value);
            value
        };
        // C s_17_4: const #12920u : u32
        let s_17_4: u32 = 12920;
        // N s_17_5: write-reg s_17_4 <= s_17_3
        let s_17_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_17_4 as isize, s_17_3);
            tracer.write_register(s_17_4 as isize, s_17_3);
        };
        // N s_17_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var r:struct
        let s_18_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_18_1: call _get_FPCR_Type_OFE(s_18_0)
        let s_18_1: bool = u_get_FPCR_Type_OFE(state, tracer, s_18_0);
        // C s_18_2: const #12920u : u32
        let s_18_2: u32 = 12920;
        // D s_18_3: read-reg s_18_2:struct
        let s_18_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_2 as isize);
            tracer.read_register(s_18_2 as isize, value);
            value
        };
        // C s_18_4: const #12920u : u32
        let s_18_4: u32 = 12920;
        // N s_18_5: write-reg s_18_4 <= s_18_3
        let s_18_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_18_4 as isize, s_18_3);
            tracer.write_register(s_18_4 as isize, s_18_3);
        };
        // N s_18_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var r:struct
        let s_19_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_19_1: call _get_FPCR_Type_UFE(s_19_0)
        let s_19_1: bool = u_get_FPCR_Type_UFE(state, tracer, s_19_0);
        // C s_19_2: const #12920u : u32
        let s_19_2: u32 = 12920;
        // D s_19_3: read-reg s_19_2:struct
        let s_19_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_2 as isize);
            tracer.read_register(s_19_2 as isize, value);
            value
        };
        // C s_19_4: const #12920u : u32
        let s_19_4: u32 = 12920;
        // N s_19_5: write-reg s_19_4 <= s_19_3
        let s_19_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_19_4 as isize, s_19_3);
            tracer.write_register(s_19_4 as isize, s_19_3);
        };
        // N s_19_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var r:struct
        let s_20_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_20_1: call _get_FPCR_Type_IXE(s_20_0)
        let s_20_1: bool = u_get_FPCR_Type_IXE(state, tracer, s_20_0);
        // C s_20_2: const #12920u : u32
        let s_20_2: u32 = 12920;
        // D s_20_3: read-reg s_20_2:struct
        let s_20_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_2 as isize);
            tracer.read_register(s_20_2 as isize, value);
            value
        };
        // C s_20_4: const #12920u : u32
        let s_20_4: u32 = 12920;
        // N s_20_5: write-reg s_20_4 <= s_20_3
        let s_20_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_20_4 as isize, s_20_3);
            tracer.write_register(s_20_4 as isize, s_20_3);
        };
        // N s_20_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var r:struct
        let s_21_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_21_1: call _get_FPCR_Type_IDE(s_21_0)
        let s_21_1: bool = u_get_FPCR_Type_IDE(state, tracer, s_21_0);
        // C s_21_2: const #12920u : u32
        let s_21_2: u32 = 12920;
        // D s_21_3: read-reg s_21_2:struct
        let s_21_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_2 as isize);
            tracer.read_register(s_21_2 as isize, value);
            value
        };
        // C s_21_4: const #12920u : u32
        let s_21_4: u32 = 12920;
        // N s_21_5: write-reg s_21_4 <= s_21_3
        let s_21_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_21_4 as isize, s_21_3);
            tracer.write_register(s_21_4 as isize, s_21_3);
        };
        // N s_21_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var r:struct
        let s_22_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_22_1: call _get_FPCR_Type_length(s_22_0)
        let s_22_1: u8 = u_get_FPCR_Type_length(state, tracer, s_22_0);
        // C s_22_2: const #12920u : u32
        let s_22_2: u32 = 12920;
        // D s_22_3: read-reg s_22_2:struct
        let s_22_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_2 as isize);
            tracer.read_register(s_22_2 as isize, value);
            value
        };
        // C s_22_4: const #12920u : u32
        let s_22_4: u32 = 12920;
        // N s_22_5: write-reg s_22_4 <= s_22_3
        let s_22_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_22_4 as isize, s_22_3);
            tracer.write_register(s_22_4 as isize, s_22_3);
        };
        // N s_22_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var r:struct
        let s_23_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_23_1: call _get_FPCR_Type_Stride(s_23_0)
        let s_23_1: u8 = u_get_FPCR_Type_Stride(state, tracer, s_23_0);
        // C s_23_2: const #12920u : u32
        let s_23_2: u32 = 12920;
        // D s_23_3: read-reg s_23_2:struct
        let s_23_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_2 as isize);
            tracer.read_register(s_23_2 as isize, value);
            value
        };
        // C s_23_4: const #12920u : u32
        let s_23_4: u32 = 12920;
        // N s_23_5: write-reg s_23_4 <= s_23_3
        let s_23_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_23_4 as isize, s_23_3);
            tracer.write_register(s_23_4 as isize, s_23_3);
        };
        // N s_23_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
