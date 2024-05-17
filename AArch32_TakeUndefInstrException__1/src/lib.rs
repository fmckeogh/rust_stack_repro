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
use ThisInstrAddr::*;
use AArch32_EnterHypMode::*;
use CurrentInstrSet::*;
use EL2Enabled::*;
use HCR_read::*;
use u_get_HCR_Type_TGE::*;
use AArch32_EnterMode::*;
use common::*;
pub fn AArch32_TakeUndefInstrException__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    except: ProductTypeb7f99f96751e17c4,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        vect_offset: i64,
        route_to_hyp: bool,
        lr_offset: i64,
        gs_10102: bool,
        preferred_exception_return: u32,
        gs_10101: bool,
        ga_7390: i64,
        except: ProductTypeb7f99f96751e17c4,
    }
    let fn_state = FunctionState {
        except,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b13 b1
        if s_0_6 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#10101 <= s_1_0
        fn_state.gs_10101 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#10101:u8
        let s_2_0: bool = fn_state.gs_10101;
        // N s_2_1: branch s_2_0 b12 b3
        if s_2_0 {
            return block_12(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#10102 <= s_3_0
        fn_state.gs_10102 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#10102:u8
        let s_4_0: bool = fn_state.gs_10102;
        // D s_4_1: write-var route_to_hyp <= s_4_0
        fn_state.route_to_hyp = s_4_0;
        // C s_4_2: const #32s : i64
        let s_4_2: i64 = 32;
        // C s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // S s_4_4: call ThisInstrAddr(s_4_3)
        let s_4_4: Bits = ThisInstrAddr(state, tracer, s_4_3);
        // S s_4_5: cast reint s_4_4 -> u32
        let s_4_5: u32 = (s_4_4.value() as u32);
        // D s_4_6: write-var preferred_exception_return <= s_4_5
        fn_state.preferred_exception_return = s_4_5;
        // C s_4_7: const #4u : u8
        let s_4_7: u8 = 4;
        // C s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 8u16);
        // C s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (s_4_8.value() as i128);
        // C s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // D s_4_11: write-var vect_offset <= s_4_10
        fn_state.vect_offset = s_4_10;
        // C s_4_12: const #() : ()
        let s_4_12: () = ();
        // S s_4_13: call CurrentInstrSet(s_4_12)
        let s_4_13: u32 = CurrentInstrSet(state, tracer, s_4_12);
        // C s_4_14: const #1u : u32
        let s_4_14: u32 = 1;
        // S s_4_15: cmp-eq s_4_13 s_4_14
        let s_4_15: bool = ((s_4_13) == (s_4_14));
        // N s_4_16: branch s_4_15 b11 b5
        if s_4_15 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #2s : i64
        let s_5_0: i64 = 2;
        // D s_5_1: write-var ga#7390 <= s_5_0
        fn_state.ga_7390 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#7390:i64
        let s_6_0: i64 = fn_state.ga_7390;
        // D s_6_1: write-var lr_offset <= s_6_0
        fn_state.lr_offset = s_6_0;
        // C s_6_2: const #16975u : u32
        let s_6_2: u32 = 16975;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // C s_6_5: const #432u : u32
        let s_6_5: u32 = 432;
        // D s_6_6: read-reg s_6_5:u8
        let s_6_6: u8 = {
            let value = state.read_register::<u8>(s_6_5 as isize);
            tracer.read_register(s_6_5 as isize, value);
            value
        };
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 2u16);
        // D s_6_8: cmp-eq s_6_4 s_6_7
        let s_6_8: bool = ((s_6_4) == (s_6_7));
        // N s_6_9: branch s_6_8 b10 b7
        if s_6_8 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var route_to_hyp:u8
        let s_7_0: bool = fn_state.route_to_hyp;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var lr_offset:i64
        let s_8_0: i64 = fn_state.lr_offset;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var vect_offset:i64
        let s_8_2: i64 = fn_state.vect_offset;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // C s_8_4: const #408u : u32
        let s_8_4: u32 = 408;
        // D s_8_5: read-reg s_8_4:u8
        let s_8_5: u8 = {
            let value = state.read_register::<u8>(s_8_4 as isize);
            tracer.read_register(s_8_4 as isize, value);
            value
        };
        // D s_8_6: read-var preferred_exception_return:u32
        let s_8_6: u32 = fn_state.preferred_exception_return;
        // D s_8_7: call AArch32_EnterMode(s_8_5, s_8_6, s_8_1, s_8_3)
        let s_8_7: () = AArch32_EnterMode(state, tracer, s_8_5, s_8_6, s_8_1, s_8_3);
        // N s_8_8: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #20u : u8
        let s_9_0: u8 = 20;
        // C s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 8u16);
        // C s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (s_9_1.value() as i128);
        // C s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: read-var except:struct
        let s_9_5: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_9_6: read-var preferred_exception_return:u32
        let s_9_6: u32 = fn_state.preferred_exception_return;
        // D s_9_7: call AArch32_EnterHypMode(s_9_5, s_9_6, s_9_4)
        let s_9_7: () = AArch32_EnterHypMode(state, tracer, s_9_5, s_9_6, s_9_4);
        // N s_9_8: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var vect_offset:i64
        let s_10_0: i64 = fn_state.vect_offset;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var except:struct
        let s_10_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_10_3: read-var preferred_exception_return:u32
        let s_10_3: u32 = fn_state.preferred_exception_return;
        // D s_10_4: call AArch32_EnterHypMode(s_10_2, s_10_3, s_10_1)
        let s_10_4: () = AArch32_EnterHypMode(state, tracer, s_10_2, s_10_3, s_10_1);
        // N s_10_5: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #4s : i64
        let s_11_0: i64 = 4;
        // D s_11_1: write-var ga#7390 <= s_11_0
        fn_state.ga_7390 = s_11_0;
        // N s_11_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call HCR_read(s_12_0)
        let s_12_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_12_0);
        // S s_12_2: call _get_HCR_Type_TGE(s_12_1)
        let s_12_2: bool = u_get_HCR_Type_TGE(state, tracer, s_12_1);
        // S s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // C s_12_4: const #1u : u8
        let s_12_4: bool = true;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // S s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // D s_12_7: write-var gs#10102 <= s_12_6
        fn_state.gs_10102 = s_12_6;
        // N s_12_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call EL2Enabled(s_13_0)
        let s_13_1: bool = EL2Enabled(state, tracer, s_13_0);
        // D s_13_2: write-var gs#10101 <= s_13_1
        fn_state.gs_10101 = s_13_1;
        // N s_13_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
